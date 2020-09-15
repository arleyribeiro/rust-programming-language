fn main() {
    let numbers: [u32; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers.iter() {
        println!(
            "The fibonacci of the number {} is {}",
            *number,
            fibonacci(*number)
        );
    }
}

fn fibonacci(number: u32) -> u32 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }
    return fibonacci(number - 1) + fibonacci(number - 2);
}
