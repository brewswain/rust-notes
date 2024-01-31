#[derive(Debug)]
enum Book {
    Length(u8),
    Title(String),
    IsFinished(bool),
}

fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        // If the value is Some, we print out the value in the Some variant by binding the value to the variable max in the pattern. We don’t want to do anything with the None value. T
        println!("The maximum is configured to be {}", max);
    }

    if_let_else_example();
} // Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

fn if_let_else_example() {
    let mut count = 0;

    let bookIsFinished = Book::IsFinished(true);

    // if count == 3 {
    //     Book::IsFinished(true);
    // }
    if let Book::IsFinished(true) = bookIsFinished {
        println!("Book series complete! Now pick up a new one :)");
    } else {
        count += 1;
    }
    println!("{:?}", bookIsFinished)
}
