#![feature(iter_collect_into)]

use ndarray::*;
use rand::prelude::*;
use rand::rng;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use adjacent_pair_iterator::AdjacentPairIterator;

const BLOCK_CHAR: char = '█';
const POINT_CHAR: char = '•';
const EMPTY_CHAR: char = ' ';

#[derive(Clone, Copy)]
struct Tile {
    up: bool,
    right: bool,
    down: bool,
    left: bool
}
impl Tile {
    fn new(walled: bool) -> Self {
        Self {
            up: walled,
            right: walled, 
            down: walled,
            left: walled,
        }
    }

    fn set_side(&mut self, direction: Direction, closed: bool) {
        match direction {
            Direction::North => self.up = closed,
            Direction::East => self.right = closed,
            Direction::South => self.down = closed,
            Direction::West => self.left = closed,
        };
    }

    fn get_mut_sides(&mut self) -> [(Direction, bool); 4] {
        [
            (Direction::North, self.up),
            (Direction::East, self.right),
            (Direction::South, self.down),
            (Direction::West, self.left),
        ]
    }

    fn get_sides(&self) -> [(Direction, bool); 4] {
        let mut mut_self: Self = Self::new(false);
        self.clone_into(&mut mut_self);
        mut_self.get_mut_sides()
    }
}

#[derive(Clone)]
struct Maze {
    size: Size,
    tiles: Array2<Tile>,
}
impl Maze {
    fn new(size: Size, walled: bool) -> Self {
        Self {
            size: size,
            tiles: Array2::from_elem(size.as_array(), Tile::new(walled)),
        }
    }

    fn generate_maze(&mut self) {
        let mut explored = vec![Position(0, 0)];
        
        let mut stack = vec![Position(0, 0)];
        
        let mut currentpos = Position(0, 0);
        
        while !(
            explored.len() != 1 &&
            currentpos == Position(0,0)
        ) {
            let dirs = self.get_valid_directions(currentpos, explored.clone());
            
            if dirs.is_empty() {
                currentpos = stack.pop().unwrap();
            } else {
                let pick = *dirs
                    .choose(&mut rng())
                    .unwrap();

                self.get_mut_tile(currentpos)
                    .unwrap()
                    .set_side(pick, false);
                
                currentpos = currentpos.translate(pick);
                
                self.get_mut_tile(currentpos)
                    .unwrap()
                    .set_side(
                        pick.get_opposite(),
                        false
                    );


                stack.push(currentpos);
                explored.push(currentpos);
            }
        }
    }

    fn get_valid_directions(&self, pos: Position, explored: Vec<Position>) -> Vec<Direction> {
        let mut invalid = vec![];
        
        if pos.0 == 0 {
            invalid.push(Direction::West);
        } else if pos.0 == self.size.get_max_pos().0 {
            invalid.push(Direction::East);
        }
        
        if pos.1 == 0 {
            invalid.push(Direction::North);
        } else if pos.1 == self.size.get_max_pos().1 {
            invalid.push(Direction::South);
        }

        
        let mut out = vec![];
        
        for direction in Direction::iter() {
            if !(
                invalid.contains(&direction) ||
                explored.contains(&pos.translate(direction))
            ) {
                out.push(direction);
            }
        }
        
        out
    }

    fn get_valid_moves(&self, pos: Position, explored: Vec<Position>) -> Vec<Direction> {
        let mut out = vec![];
        
        let invalid: Vec<Direction> = self
            .get_tile(pos)
            .unwrap()
            .get_sides().iter()
            .filter_map(
                |(a,b)| if *b {
                    Some(*a)
                } else { None }
            ).collect();

        
        for direction in Direction::iter() {
            if !(
                invalid.contains(&direction) ||
                explored.contains(&pos.translate(direction)
            )) {
                out.push(direction);
            }
        }
        
        out
    }

    fn solve_maze(&self) -> Vec<Position> { // Depth-First Search (DFS)
        let goal = self.size.get_max_pos();

        let mut explored = vec![Position::new()];
        let mut path = vec![Position::new()];
        
        let mut currentpos = Position::new();

        
        let mut popped = false;
        
        while currentpos != goal {
            let moves = self.get_valid_moves(currentpos, explored.clone());
            
            if moves.is_empty() {
                currentpos = path.pop().unwrap();
                
                popped = true;
            } else {
                if popped {
                    path.push(currentpos);
                }
                
                let direction = *moves.choose(&mut rng()).unwrap();
                
                currentpos = currentpos.translate(direction);

                path.push(currentpos);
            }
            
            explored.push(currentpos);
        }

        
        path.dedup();
        path
    }

    fn to_display_pos(pos: Position) -> Position {
        Position::from_array(
            pos.as_array()
            .map( |x| x * 2 + 1 )
        )
    }

    fn get_tile(&self, pos: Position) -> Option<&Tile> {
        self.tiles.get(pos.as_array())
    }

    fn get_mut_tile(&mut self, pos: Position) -> Option<&mut Tile> {
        self.tiles.get_mut(pos.as_array())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn get_axis(&self) -> Axis {
        match self {
            Self::East | Self::West => Axis(0),
            Self::North | Self::South => Axis(1),
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn get_perpendiculars(&self) -> [Self; 2] {
        match self.get_axis() {
            Axis(0) => [Self::North, Self::South],
            Axis(1) => [Self::East, Self::West],
            _ => panic!("Higher Axis"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);
impl Position {
    fn new() -> Self {
        Self(0, 0)
    }

    fn as_array(&self) -> [usize; 2] {
        [self.0, self.1]
    }

    fn as_rev_array(&self) -> [usize; 2] {
        [self.1, self.0]
    }
    
    fn from_array(arr: [usize; 2]) -> Self {
        Self(arr[0], arr[1])
    }

    fn translate(&self, direction: Direction) -> Self {
        let mut out = *self;
        
        match direction {
            Direction::North => out.1 -= 1,
            Direction::East => out.0 += 1,
            Direction::South => out.1 += 1,
            Direction::West => out.0 -= 1,
        };
        
        out
    }

    fn from_size(size: Size) -> Self {
        Self(size.0, size.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Size(usize, usize);
impl Size {
    fn as_array(&self) -> [usize; 2] {
        [self.0, self.1]
    }

    fn as_rev_array(&self) -> [usize; 2] {
        [self.1, self.0]
    }

    fn from_array(arr: [usize; 2]) -> Self {
        Self(arr[0], arr[1])
    }

    fn get_max_pos(&self) -> Position {
        Position(self.0 - 1, self.1 - 1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    origin: Position,
    direction: Direction,
    magnitude: usize,
}
impl Vector {
    fn new(origin: Position, direction: Direction, magnitude: usize) -> Self {
        Vector {
            origin: origin,
            direction: direction,
            magnitude: magnitude,
        }
    }

    fn new_from_points(origin: Position, end: Position) -> Result<Self, io::ErrorKind> {
        let mut magnitude: Vec<isize> = vec![];
        end.as_array()
            .iter()
            .zip(origin.as_array())
            .map(|(a,b)| *a as isize - b as isize)
            .collect_into(&mut magnitude);

        
        let mut unit: Vec<isize> = vec![];
        magnitude.iter()
            .map(|x| x.signum())
            .collect_into(&mut unit);
        
        let unit = (unit[0], unit[1]);

        
        let direction = match unit {
            (0,-1) => Ok(Direction::North),
            (1,0) => Ok(Direction::East),
            (0,1) => Ok(Direction::South),
            (-1,0) => Ok(Direction::West),
            _ => Err(io::ErrorKind::InvalidInput)
        }?;

        let magnitude: usize = magnitude.iter()
            .map(|x| x.abs() as usize)
            .reduce(|a,b| a+b)
            .unwrap() + 1;
        
        Ok(Self::new(origin, direction, magnitude))
    }

    fn get_end(&self) -> Position {
        let origin = self.origin;
        
        let magnitude = self.magnitude - 1;
    
        match self.direction {
            Direction::North => Position(
                origin.0, 
                origin.1 - magnitude
            ),
            Direction::East => Position(
                origin.0 + magnitude,
                origin.1
            ),
            Direction::South => Position(
                origin.0,
                origin.1 + magnitude
            ),
            Direction::West => Position(
                origin.0 - magnitude, 
                origin.1
            ),
        }
    }
}

struct Rectangle {
    origin: Position,
    size: Size,
}
impl Rectangle {
    fn new(origin: Position, size: Size) -> Self {
        Rectangle {
            origin: origin,
            size: size,
        }
    }

    fn get_vectors(&self) -> [Vector; 4] {
        let max_pos = Position::from_size(self.size);
        
        let right = Vector::new(
            self.origin,
            Direction::East,
            max_pos.0
        );
        
        let down = Vector::new(
            self.origin,
            Direction::South,
            max_pos.1
        );
        
        [
            right,
            down,
            Vector::new(
                down.get_end(),
                Direction::East,
                max_pos.0
            ),
            Vector::new(
                right.get_end(),
                Direction::South,
                max_pos.1
            ),
        ]
    }
}

struct Display {
    origin: Position,
    pixels: Array2<char>,
    size: Size,
}
impl Display {
    fn new(origin: Position, size: Size) -> Display {
        Display {
            origin: origin,
            pixels: Array2::from_elem(
                size.as_rev_array(),
                EMPTY_CHAR
            ),
            size: size,
        }
    }

    fn new_from_maze(origin: Position, maze: Maze) -> Self {
        let size = Size::from_array(maze.size.as_array().map(|x| x * 2 + 1));
        
        Self::new(origin, size)
    }

    fn print(&self) {
        print!("{}", "\n".repeat(self.origin.1));
        
        for row in self.pixels.rows() {
            let mut rowstring = String::new();
            
            for pixel in row {
                rowstring.push(*pixel);
            }

            
            print!("{}", " ".repeat(self.origin.0));
            println!("{}", rowstring);
        }
    }

    fn draw_line(&mut self, line: Vector, symbol: char) {
        let axis = line.direction.get_axis();

        match axis {
            Axis(0) => {
                let mut row = self.pixels.row_mut(line.origin.1);
                
                if line.get_end().0 > line.origin.0 {
                    for i in line.origin.0..=line.get_end().0 {
                        row[i] = symbol;
                    }
                } else {
                    for i in line.get_end().0..=line.origin.0 {
                        row[i] = symbol;
                    }
                }
            },
            Axis(1) => {
                let mut column = self.pixels.column_mut(line.origin.0);
                
                if line.get_end().1 > line.origin.1 {
                    for i in line.origin.1..=line.get_end().1 {
                        column[i] = symbol;
                    }
                } else {
                    for i in line.get_end().1..=line.origin.1 {
                        column[i] = symbol;
                    }
                }
            },
            _ => panic!("Display dimensions too high!"),
        }
    }

    fn draw_rect(&mut self, rectangle: Rectangle, symbol: char) {
        for vector in rectangle.get_vectors() {
            self.draw_line(vector, symbol);
        }
    }

    fn draw_maze(&mut self, maze: Maze) -> Result<(), io::ErrorKind> {
        let req_maze_size = Self::new_from_maze(self.origin, maze.clone()).size;
        if self.size == req_maze_size {
            self.draw_rect(
                Rectangle::new(
                    Position::new(),
                    self.size
                ),
                BLOCK_CHAR
            );

            
            for ((x, y), tile) in maze.tiles.indexed_iter() {
                let pos = Position(x,y);
                let display_pos = Maze::to_display_pos(pos);

                
                for (direction, wall) in tile.get_sides() {
                    if wall {
                        let perpendicular = direction.get_perpendiculars()[0];
                        
                        self.draw_line(
                            Vector::new(
                                display_pos.translate(direction).translate(perpendicular),
                                perpendicular.get_opposite(),
                                3
                            ),
                            BLOCK_CHAR
                        );
                    }
                }
            }
            
            return Ok(());
        }
        
        Err(io::ErrorKind::InvalidInput)
    }

    fn draw_path(&mut self, path: Vec<Position>, symbol: char) -> Result<(), io::ErrorKind> {
        for (a,b) in path.adjacent_pairs() {
            let vector = Vector::new_from_points(a,b)?;
            
            self.draw_line(vector, symbol);
        }
        
        Ok(())
    }

    fn draw_point(&mut self, pos: Position, symbol: char) {
        self.pixels[pos.as_rev_array()] = symbol;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    const INVALID_INPUT: &str = "Pass the dimension of your desired maze with 'AxY' (example: '10x20')";
    
    if args.len() != 2 {
        panic!("{}", INVALID_INPUT);
    }
    
    let size = args[1].split_once("x").expect(INVALID_INPUT);
    let size = Size(str::parse(size.0).expect(INVALID_INPUT), str::parse(size.1).expect(INVALID_INPUT));


    let mut maze = Maze::new(size, true);
    maze.generate_maze();

    let mut display = Display::new_from_maze(Position(1,1), maze.clone());
    display.draw_maze(maze.clone()).unwrap();
    
    display.draw_path(
        maze.solve_maze()
            .iter()
            .map(|x| Maze::to_display_pos(*x))
            .collect(),
        POINT_CHAR
    ).unwrap();

    display.draw_point(Position(1,0), POINT_CHAR);
    display.draw_point(
        display.size
            .get_max_pos()
            .translate(Direction::West),
        POINT_CHAR
    );

    display.print();
}