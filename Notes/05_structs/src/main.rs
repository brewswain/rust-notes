fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect2 = (70, 10);

    println!(
        "The area of the rectangle is {} square pixels.",
        area((width1, height1))
    );
    println!(
        "The area of the second rectangle is {} square pixels.",
        area(rect2)
    );

    calculate_area_using_structs();
}

fn area(dimensions: (u32, u32)) -> u32 {
    let height = dimensions.0;
    let width = dimensions.1;

    height * width
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_area_using_structs() {
    let rect = Rectangle {
        width: 320,
        height: 50,
    };

    println!(
        "calculate_area_using_structs: The area of the rectangle is {} square pixels.",
        rect_area(&rect)
    );
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
