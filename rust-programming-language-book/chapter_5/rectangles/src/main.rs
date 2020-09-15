mod area_tuples;
mod area_variables;
mod rectangle;

fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    let rect1 = (30, 50);

    let rect2 = rectangle::create(width1, height1);

    // Calculating the area of a rectangle specified by separate width and height variables
    println!(
        "The area of the rectangle is {} square pixels.",
        area_variables::area(width1, height1)
    );

    // Specifying the width and height of the rectangle with a tuple
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples::area(rect1)
    );

    // Defining a Rectangle struct
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle::area(&rect2)
    );
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
}
