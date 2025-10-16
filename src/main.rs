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
struct Position {x: usize, y: usize}
impl Position {
  fn new(x: usize, y: usize) -> Self {
    Position { x: x, y: y }
  }

  fn as_array(&self) -> [usize; 2] {
    [self.x, self.y]
  }
}

struct Size {width: usize, height: usize}
impl Size {
  fn new(width: usize, height: usize) -> Self {
    Size { width: width, height: height }
  }

  fn as_array(&self) -> [usize; 2] {
    [self.width, self.height]
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
      CardinalDirection::North => Position::new(origin.x, origin.y - magnitude),
      CardinalDirection::East => Position::new(origin.x + magnitude, origin.y),
      CardinalDirection::South => Position::new(origin.x, origin.y + magnitude),
      CardinalDirection::West => Position::new(origin.x - magnitude, origin.y)
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
    let right = Vector::new(self.origin, CardinalDirection::East, self.size.width);
    let down = Vector::new(self.origin, CardinalDirection::South, self.size.height);
    [
      right,
      down,
      Vector::new(down.get_end(), CardinalDirection::East, self.size.width),
      Vector::new(right.get_end(), CardinalDirection::South, self.size.height)
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
      let mut row = self.pixels.row_mut(line.origin.y);
      for i in line.origin.x..=line.get_end().x {
        row[i] = true;
      }
    } else {
      let mut column = self.pixels.column_mut(line.origin.x);
      for i in line.origin.y..=line.get_end().y {
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
  let mut display = Display::new(Size::new(10, 10));
  display.draw_rect(Rectangle::new(Position::new(1, 1), Size::new(5, 5)));
  display.print();
}