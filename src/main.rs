#![allow(non_snake_case)]

use ndarray::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone)]
struct Tile {
    sides: HashMap<CardinalDirection, bool>,
}
impl Tile {
    fn new() -> Self {
        let mut sides = HashMap::new();

        for side in CardinalDirection::iter() {
            sides.insert(side, false);
        }
        Self { sides: sides }
    }
}

struct Maze {
    tiles: Array2<Tile>,
    solvedPath: Vec<Position>,
}

impl Maze {
    fn new(size: Size) -> Self {
        Self {
            tiles: Array2::from_elem(size.as_array(), Tile::new()),
            solvedPath: vec![Position(0, 0)],
        }
    }

    fn get_adj_tiles(&self, pos: Position) -> HashMap<CardinalDirection, &Tile> {
        HashMap::from([
            (CardinalDirection::North, &self.tiles[[pos.0, pos.1 - 1]]),
            (CardinalDirection::East, &self.tiles[[pos.0 + 1, pos.1]]),
            (CardinalDirection::South, &self.tiles[[pos.0, pos.1 + 1]]),
            (CardinalDirection::West, &self.tiles[[pos.0 - 1, pos.1]]),
        ])
    }

    fn get_req_walls(&self, pos: Position) -> HashMap<CardinalDirection, bool> {
        let mut out = HashMap::<CardinalDirection, bool>::new();
        for (direction, tile) in self.get_adj_tiles(pos) {
            out.insert(
                direction,
                *tile.sides.get(&direction.get_opposite()).unwrap(),
            );
        }
        out
    }

    fn new_Tile(&mut self, pos: Position, tile: Tile) -> Tile {
        let mut newTile = tile;
        let req_walls = self.get_req_walls(pos);
        for (side, wall) in newTile.sides.iter_mut() {
            if *req_walls.get(&side).unwrap() {
                *wall = true;
            }
        }
        newTile
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}
impl CardinalDirection {
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
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);
impl Position {
    fn new() -> Self {
        Position(0, 0)
    }
    fn as_array(&self) -> [usize; 2] {
        [self.0, self.1]
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

struct Size(usize, usize);
impl Size {
    fn new(size: usize) -> Self {
        Size(size, size)
    }
    fn as_array(&self) -> [usize; 2] {
        [self.0, self.1]
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    origin: Position,
    direction: CardinalDirection,
    magnitude: usize,
}
impl Vector {
    fn new(origin: Position, direction: CardinalDirection, magnitude: usize) -> Self {
        Vector {
            origin: origin,
            direction: direction,
            magnitude: (magnitude),
        }
    }

    fn get_end(&self) -> Position {
        let origin = self.origin - 1;
        let magnitude = self.magnitude - 1;
        match self.direction {
            CardinalDirection::North => Position(origin.0, origin.1 - magnitude),
            CardinalDirection::East => Position(origin.0 + magnitude, origin.1),
            CardinalDirection::South => Position(origin.0, origin.1 + magnitude),
            CardinalDirection::West => Position(origin.0 - magnitude, origin.1),
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
        let width = self.size.0;
        let height = self.size.1;
        let right = Vector::new(self.origin, CardinalDirection::East, width);
        let down = Vector::new(self.origin, CardinalDirection::South, height);
        [
            right,
            down,
            Vector::new(down.get_end(), CardinalDirection::East, width),
            Vector::new(right.get_end(), CardinalDirection::South, height),
        ]
    }
}

struct Display {
    origin: Position,
    pixels: Array2<bool>,
}
impl Display {
    fn new(origin: Position, size: Size) -> Display {
        Display {
            origin: origin,
            pixels: Array2::from_elem(size.as_array(), false),
        }
    }

    fn print(&self) {
        print!("{}", "\n".repeat(self.origin.1));
        for row in self.pixels.rows() {
            let mut rowstring = String::new();
            for pixel in row {
                if *pixel {
                    rowstring.push('#');
                } else {
                    rowstring.push(' ');
                }
            }
            print!("{}", " ".repeat(self.origin.0));
            println!("{}", rowstring);
        }
    }

    fn draw_line(&mut self, line: Vector) {
        let axis = line.direction.get_axis();
        if axis == Axis(0) {
            let mut row = self.pixels.row_mut(line.origin.1);
            for i in line.origin.0..=line.get_end().0 {
                row[i] = true;
            }
        } else {
            let mut column = self.pixels.column_mut(line.origin.0);
            for i in line.origin.1..=line.get_end().1 {
                column[i] = true;
            }
        }
    }

    fn draw_rect(&mut self, rectangle: Rectangle) {
        for vector in rectangle.get_vectors() {
            self.draw_line(vector);
        }
    }
}

fn main() {
    let mut display = Display::new(Position(1, 1), Size::new(10));
    display.draw_rect(Rectangle::new(Position::new(), Size::new(10)));
    display.print();
}
