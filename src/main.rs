use ndarray::*;

#[derive(Clone, Copy)]
enum CardinalDirection {
  North,
  East,
  South,
  West
}
impl CardinalDirection {
  fn get_axis(&self) -> Axis {
    match self {
      Self::East | Self::West => Axis(0),
      Self::North | Self::South => Axis(1)
    }
  }
}

#[derive(Clone, Copy)]
struct Position (usize, usize);
impl Position {
  fn new() -> Self {
    Position(0, 0)
  }
  fn as_array(&self) -> [usize; 2] {
    [self.0, self.1]
  }
}

struct Size (usize, usize);
impl Size {
  fn new(size: usize) -> Self {
    Size(size, size)
  }
  fn as_array(&self) -> [usize; 2] {
    [self.0, self.1]
  }
}

#[derive(Clone, Copy)]
struct Vector {
  origin: Position,
  direction: CardinalDirection,
  magnitude: usize
}
impl Vector {
  fn new(origin: Position, direction: CardinalDirection, magnitude: usize) -> Self {
    Vector { origin: origin, direction: direction, magnitude: magnitude }
  }

  fn get_end(&self) -> Position {
    let origin = &self.origin;
    let magnitude = &self.magnitude;
    match self.direction {
      CardinalDirection::North => Position(origin.0, origin.1 - magnitude),
      CardinalDirection::East => Position(origin.0 + magnitude, origin.1),
      CardinalDirection::South => Position(origin.0, origin.1 + magnitude),
      CardinalDirection::West => Position(origin.0 - magnitude, origin.1)
    }
  }
}

struct Rectangle {
  origin: Position,
  size: Size
}
impl Rectangle {
  fn new(origin: Position, size: Size) -> Self {
    Rectangle { origin: origin, size: size }
  }

  fn get_vectors(&self) -> [Vector; 4] {
    let width = self.size.0;
    let height = self.size.1;
    let right = Vector::new(self.origin, CardinalDirection::East, self.size.0);
    let down = Vector::new(self.origin, CardinalDirection::South, self.size.1);
    [
      right,
      down,
      Vector::new(down.get_end(), CardinalDirection::East, self.size.0),
      Vector::new(right.get_end(), CardinalDirection::South, self.size.1)
    ]
  }
}


struct Display {
  pixels: Array2<bool>,
  size: Size
}
impl Display {
  fn new(size: Size) -> Display {
    Display {
      pixels: Array2::from_elem(size.as_array(), false),
      size: size
    }
  }

  fn print(&self) {
    for row in self.pixels.rows() {
      let mut rowstring = String::new();
      for pixel in row {
        if *pixel { rowstring.push('#'); } else { rowstring.push(' '); }
      }
      println!("{}", rowstring);
    }
  }
  
  fn draw_line(&mut self, line: Vector) {
    let axis = line.direction.get_axis();
    if axis == Axis(0) {
      let mut row = self.pixels.row_mut(line.origin.1);
      for i in line.origin.0..line.get_end().0 {
        row[i] = true;
      }
    } else {
      let mut column = self.pixels.column_mut(line.origin.0);
      for i in line.origin.1..line.get_end().1 {
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
  let mut display = Display::new(Size::new(10));
  display.draw_rect(Rectangle::new(Position::new(), Size::new(10)));
  display.print();
}