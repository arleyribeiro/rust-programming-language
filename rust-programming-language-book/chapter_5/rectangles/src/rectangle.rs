#[derive(Debug)]
pub struct Rectangle {
  width: u32,
  height: u32,
}

pub fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

pub fn create(width: u32, height: u32) -> Rectangle {
  Rectangle { height, width }
}
