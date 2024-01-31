// when you put data on the heap, you request a certain amount of space.
// The memory allocator finds an empty spot in the heap that is big enough,
// marks it as being in use, and returns a pointer, which is the address of
// that location.

fn main() {
    let string_literal = "hi"; // this is a string literal, which is stored on the stack
    let mut string_on_heap = String::from("Hello"); // This type manages data allocated to the heap and is able to store an amount of text that is unknown to us at compile time.

    // The double colon :: operator allows us to namespace this particular from function under the String
    //  type rather than using some sort of name like string_from.

    println!("string literal: {string_literal}");
    string_on_heap.push_str(", world!");
    println!("{}", string_on_heap);

    example_one();
    example_two();
    ownership_example();

    // return values and scope block
    return_values_scope_and_ownership();
    return_multiple_values();
}

// String can be mutated (Note the capital S), but string literals can't.

// In the case of a string literal, since we know the contents at compile time, the text is hardcoded directly into the final executable. This causes string literals to be super efficient, but this comes due to the string literal's immutability.
// Unfortunately, we can't place a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

// With the String type, however, due to it being a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
// This means 2 things:
// - The memory must be requested from the memory allocator at runtime (Malloc vibes)
// - The memory needs to be returned to the allocator when we're done with our string.

// We do the first part when we call `String::from`. When we call it, its implementation requests the memory it needs.
// For the second part, rust takes a unique option to make life easier for us: The memory gets automatically
// returned once the variable that owns it goes out of scope. look at our example below:

fn example_one() {
    let mut is_active = true;
    while is_active {
        let s = String::from("Hello"); // s is valid from this point forward

        // do stuff with s
        let s2 = s.clone();
        // using .clone() also deeply copies the heap data of the String in question.
        println!("example_one: s1 = {}, s2 = {}", s, s2);
        is_active = false;
    }
    // From here on out, now s is no longer valid.
    // When a variable goes out of scope, Rust calls a special function for us, called `drop`. This is where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

    // Visit this link for more details: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
}

fn example_two() {
    let x = 5;
    let y = x;

    // Weirdly enough, in this case we don't call the `clone` method, but this is valid. The reason is that integers are known size at compile time and are stored directly on the stack. This means that copies of the actual values are quick to make. In other words, there's no difference between deep and shallow copying in this situation.

    // the types that implement the `Copy` trait are stored on the stack. These types are:
    // - Integer types such as `u32`
    // - The boolean type `bool`
    // - floating point types, such as `f64`
    // - The character type `char`
    // - Tuples, but only if they contain types that implement Copy

    println!("example_two: x = {}, y = {}", x, y);
}

fn ownership_example() {
    let s = String::from("ownership_example: hello"); // s comes into scope
    takes_ownership(s); // s' value moves into the function, therefore s is no longer valid here.

    // if i tried to use `s` from this point onwards, we;d get a compile error

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but since it's i32 and therefore stored in the stack, it's still okay to still use x afterward.
}

fn takes_ownership(string_param: String) {
    // string_param comes into scope here
    println!("takes_ownership: {}", string_param);
} // string_param goes out of scope, calling `drop`. The backing memory is freed

fn makes_copy(int_param: i32) {
    // int_param comes into scope
    println!("makes_copy: {}", int_param);
} // Here, int_param goes out of scope. Nothing special happens

fn return_values_scope_and_ownership() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("return_values_scope_and_ownership: hello"); // s2 comes into scope

    let s3 = takes_and_gives_back_ownership(s2); // s2 is moved into takes_and_gives_back_ownership, which also moves its return value into s3.
} // s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("gives_ownership: yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back_ownership(string_param: String) -> String {
    // string_param comes into scope
    string_param // string_param is returned and moves out to the calling function
}

// The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

// While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

// Rust does let us return multiple values using a tuple:

fn return_multiple_values() {
    let s1 = String::from("return_multiple_values: hello");

    let (s2, len) = calculate_length(s1);

    println!("calculate_length: The length of '{}' is {}.", s2, len);
}

fn calculate_length(string_param: String) -> (String, usize) {
    let length = string_param.len(); // len() returns the length of a String

    (string_param, length)
}

// But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called references.
