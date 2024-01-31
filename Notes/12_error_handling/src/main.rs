use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("Crash and burn");

    // use backtraces with command RUST_BACKTRACE=1 cargo run:
    // let v = vec![1, 2, 3];
    // v[99];
    // recoverable_error();
    // recoverable_error_with_result();
    shortcuts_for_panic();
    propagating_errors();
}

// Re-enable when not using propagating_errors
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn recoverable_error() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        // Like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn recoverable_error_with_result() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("error_handler: Problem creating the file: {:?}", error);
            })
        } else {
            panic!("error_handler: Problem opening the file: {:?}", error)
        }
    });
} // unlike recoverable_error(), this doesn’t contain any match expressions and is cleaner to read.

fn shortcuts_for_panic() {
    // Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The Result<T, E> type has many helper methods defined on it to do various, more specific tasks. The unwrap method is a shortcut method.  If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.

    // let greeting_file_unwrap = File::open("hello.txt").unwrap();
    // println!("{:?}", greeting_file_unwrap);

    let greeting_file_expect = File::open("hello.txt")
        .expect("error_handler: hello.txt should be included in this project"); // In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.
}

fn propagating_errors() {
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_concise_version() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in read_username_from_file(). If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

    // error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert values from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_even_more_concise() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_most_concise() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
} // Reading a file into a string is a fairly common operation, so the standard library provides the convenient fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.
