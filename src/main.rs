#![allow(non_snake_case)]
#![feature(iter_collect_into)]

use ndarray::*;
use rand::prelude::*;
use rand::rng;
use std::collections::HashMap;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use adjacent_pair_iterator::AdjacentPairIterator;

const BLOCK_CHAR: char = '█';
const POINT_CHAR: char = '•';

#[derive(Clone)]
struct Tile {
    sides: HashMap<Direction, bool>,
}
impl Tile {
    fn new(walled: bool) -> Self {
        let mut sides = HashMap::new();

        for side in Direction::iter() {
            sides.insert(side, walled);
        }
        Self { sides: sides }
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
        loop {
            let dirs = self.get_valid_directions(currentpos, explored.clone());
            if dirs.is_empty() {
                currentpos = stack.pop().unwrap();
            } else {
                let pick = *dirs.choose(&mut rng()).unwrap();
                self.tiles[currentpos.as_array()].sides.insert(pick, false);
                currentpos = currentpos.translate(pick);
                self.tiles[currentpos.as_array()]
                    .sides
                    .insert(pick.get_opposite(), false);
                stack.push(currentpos);
                explored.push(currentpos);
            }
            if currentpos == Position::new() {
                break;
            }
        }
    }

    fn get_valid_directions(&self, pos: Position, explored: Vec<Position>) -> Vec<Direction> {
        let mut invalid = vec![];
        if pos.0 == 0 {
            invalid.push(Direction::West);
        }
        if pos.1 == 0 {
            invalid.push(Direction::North);
        }
        if pos.0 == self.size.0 - 1 {
            invalid.push(Direction::East);
        }
        if pos.1 == self.size.1 - 1 {
            invalid.push(Direction::South);
        }
        let mut out = vec![];
        for direction in Direction::iter() {
            if !invalid.contains(&direction) && !explored.contains(&pos.translate(direction)) {
                out.push(direction);
            }
        }
        out
    }

    fn get_valid_moves(&self, pos: Position, explored: Vec<Position>) -> Vec<Direction> {
        let mut out = vec![];
        let invalid: Vec<Direction> = self
            .tiles[pos.as_array()]
            .sides.iter()
            .filter_map(
                |(a,b)| if *b {
                    Some(*a)
                } else { None }
            ).collect();
        for direction in Direction::iter() {
            if !invalid.contains(&direction) && !explored.contains(&pos.translate(direction)) {
                out.push(direction);
            }
        }
        out
    }

    fn get_adj_tiles(&self, pos: Position) -> HashMap<Direction, Tile> {
        let mut out = HashMap::new();
        if pos.0 == 0 {
            out.insert(Direction::West, Tile::new(false));
        }
        if pos.1 == 0 {
            out.insert(Direction::North, Tile::new(false));
        }
        if pos.0 == self.size.0 - 1 {
            out.insert(Direction::East, Tile::new(false));
        }
        if pos.1 == self.size.1 - 1 {
            out.insert(Direction::South, Tile::new(false));
        }
        for direction in Direction::iter() {
            if !out.contains_key(&direction) {
                out.insert(
                    direction,
                    match direction {
                        Direction::North => self.tiles[[pos.0, pos.1 - 1]].clone(),
                        Direction::East => self.tiles[[pos.0 + 1, pos.1]].clone(),
                        Direction::South => self.tiles[[pos.0, pos.1 + 1]].clone(),
                        Direction::West => self.tiles[[pos.0 - 1, pos.1]].clone(),
                    },
                );
            }
        }
        out
    }

    fn get_req_walls(&self, pos: Position) -> HashMap<Direction, bool> {
        let mut out = HashMap::<Direction, bool>::new();
        for (direction, tile) in self.get_adj_tiles(pos) {
            out.insert(
                direction,
                *tile.sides.get(&direction.get_opposite()).unwrap(),
            );
        }
        out
    }

    fn new_Tile(&self, pos: Position, tile: Tile) -> Tile {
        let mut newTile = tile;
        let req_walls = self.get_req_walls(pos);
        for (side, wall) in newTile.sides.iter_mut() {
            if *req_walls.get(&side).unwrap() {
                *wall = true;
            }
        }
        newTile
    }

    fn solve_maze(&self) -> Vec<Position> { // Depth-First Search (DFS)
        let goal = Position(self.size.0 - 1, self.size.1 - 1);
        let mut path = vec![Position::new()];
        let mut explored = vec![Position::new()];
        let mut currentpos = Position::new();
        let mut popped = false;
        while currentpos != goal {
            let moves = self.get_valid_moves(currentpos, explored.clone());
            if moves.is_empty() {
                currentpos = path.pop().unwrap();
                popped = true;
            } else {
                if popped { path.push(currentpos); }
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
        Position(
            pos.0 * 2 + 1,
            pos.1 * 2 + 1
        )
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

    fn get_perpendicular(&self) -> [Self; 2] {
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
        Position(0, 0)
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
}
impl std::ops::Sub<usize> for Position {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        let mut out = Self::new();
        if self.0 == 0 {
            out.0 = 0;
        } else {
            out.0 = self.0 + rhs;
        }
        if self.1 == 0 {
            out.1 = 0;
        } else {
            out.1 = self.1 - rhs;
        }
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Size(usize, usize);
impl Size {
    fn new(size: usize) -> Self {
        Size(size, size)
    }
    
    fn as_array(&self) -> [usize; 2] {
        [self.0, self.1]
    }

    fn as_rev_array(&self) -> [usize; 2] {
        [self.1, self.0]
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
            Direction::North => Position(origin.0, origin.1 - magnitude),
            Direction::East => Position(origin.0 + magnitude, origin.1),
            Direction::South => Position(origin.0, origin.1 + magnitude),
            Direction::West => Position(origin.0 - magnitude, origin.1),
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
        let width = self.size.0 - 1;
        let height = self.size.1 - 1;
        let right = Vector::new(self.origin, Direction::East, width);
        let down = Vector::new(self.origin, Direction::South, height);
        [
            right,
            down,
            Vector::new(down.get_end(), Direction::East, width),
            Vector::new(right.get_end(), Direction::South, height),
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
            pixels: Array2::from_elem(size.as_rev_array(), ' '),
            size: size,
        }
    }

    fn new_from_maze(origin: Position, maze: Maze) -> Self {
        let size = Size(maze.size.0 * 2 + 2, maze.size.1 * 2 + 2);
        Display {
            origin: origin,
            pixels: Array2::from_elem(size.as_rev_array(), ' '),
            size: size,
        }
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
        if axis == Axis(0) {
            let mut row = self.pixels.row_mut(line.origin.1);
            if line.get_end().0 > line.origin.0 {
                for i in line.origin.0..=line.get_end().0 { row[i] = symbol; }
            } else {
                for i in line.get_end().0..=line.origin.0 { row[i] = symbol; }
            }
        } else {
            let mut column = self.pixels.column_mut(line.origin.0);
            if line.get_end().1 > line.origin.1 {
                for i in line.origin.1..=line.get_end().1 { column[i] = symbol; }
            } else {
                for i in line.get_end().1..=line.origin.1 { column[i] = symbol; }
            }
        }
    }

    fn draw_rect(&mut self, rectangle: Rectangle) {
        for vector in rectangle.get_vectors() {
            self.draw_line(vector, BLOCK_CHAR);
        }
    }

    fn draw_maze(&mut self, maze: Maze) -> Result<(), io::ErrorKind> {
        if Self::new_from_maze(self.origin, maze.clone()).size == self.size {
            self.draw_rect(Rectangle::new(Position(0, 0), Size(self.size.0, self.size.1)));
            for ((x, y), tile) in maze.tiles.indexed_iter() {
                let display_pos = Maze::to_display_pos(Position(x, y));
                for (direction, wall) in tile.sides.clone() {
                    if wall {
                        let perpendicular = direction.get_perpendicular()[0];
                        self.draw_line(Vector::new(
                            display_pos.translate(direction).translate(perpendicular),
                            perpendicular.get_opposite(),
                            3
                        ), BLOCK_CHAR);
                    }
                }
            }
            Ok(())
        } else {
            Err(io::ErrorKind::InvalidInput)
        }
    }

    fn draw_path(&mut self, path: Vec<Position>) -> Result<(), io::ErrorKind> {
        for (a,b) in path.adjacent_pairs() {
            let vector = Vector::new_from_points(a,b)?;
            self.draw_line(vector, POINT_CHAR);
            self.draw_point(Position(1,0), POINT_CHAR);
            self.draw_point(Position(self.size.0 - 3, self.size.1 - 2), POINT_CHAR);
        }
        Ok(())
    }

    fn draw_point(&mut self, pos: Position, symbol: char) {
        self.pixels[pos.as_rev_array()] = symbol;
    }
}

fn main() {
    let mut maze = Maze::new(Size(50,10), true);
    let mut display = Display::new_from_maze(Position(1,1), maze.clone());
    maze.generate_maze();
    display.draw_maze(maze.clone()).unwrap();
    let solve: Vec<Position> = maze.solve_maze()
        .iter()
        .map(|x| Maze::to_display_pos(*x))
        .collect();
    display.draw_path(solve).unwrap();
    display.print();
}