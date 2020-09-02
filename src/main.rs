mod print;

fn main() {
    print::run();

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Jonh", activity="Baseball");
}
