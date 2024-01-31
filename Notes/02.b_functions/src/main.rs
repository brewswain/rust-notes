fn main() {
    let x = five();
    println!("The value of x is: {x}");
    another_function(25);
}

fn another_function(y: i32) {
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
