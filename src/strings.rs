// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
  // Default str
  let hello = "Hello";
  println!("Length of hello: {}", hello.len());

  // hello.push('W'); // no is possible change the value of variables hello

  let mut hi = String::from("Hi");
  println!("Length of hi: {}", hi.len());

  hi.push('W'); // push to one character
  hi.push_str(" we"); // push more than one character
  println!("{}", hi);

  // Capacity in bytes
  println!("{}", hi.capacity());

  // Check if empty
  println!("Is empty: {}", hi.is_empty());

  // Contains
  println!("Contains 'Hi' {}", hi.contains("Hi"));

  // Replace
  println!("Replace: {}", hi.replace("Hi", "TA"));

  // Loop through string by whitespace
  for word in hi.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());

}