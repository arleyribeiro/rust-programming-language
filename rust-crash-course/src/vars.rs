// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Brad";
  let mut age = 29; // turn possible mutable the value of variable
  println!("My name is {} and I am {}", name, age);
  age = 30;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Brand", 29);
  println!("My name is {} and I am {}", my_name, my_age);
}