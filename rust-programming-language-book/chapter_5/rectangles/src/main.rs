mod area_tuples;
mod area_variables;
mod rectangle;

fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    let rect_tuple = (30, 50);

    let rect = rectangle::create(width1, height1);

    // Calculating the area of a rectangle specified by separate width and height variables
    println!(
        "The area of the rectangle is {} square pixels.",
        area_variables::area(width1, height1)
    );

    // Specifying the width and height of the rectangle with a tuple
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples::area(rect_tuple)
    );

    // Defining a Rectangle struct
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle::area(&rect)
    );
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = rectangle::create(30, 50);
    let rect2 = rectangle::create(10, 40);
    let rect3 = rectangle::create(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
