#[derive(Debug)]
pub struct Rectangle {
  width: u32,
  height: u32,
}

// Defining an area method on the Rectangle struct
impl Rectangle {
  pub fn area(&self) -> u32 {
    self.width * self.height
  }
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
  pub fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

pub fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

pub fn create(width: u32, height: u32) -> Rectangle {
  Rectangle { height, width }
}
