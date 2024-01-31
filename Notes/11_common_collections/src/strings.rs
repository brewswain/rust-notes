// We discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

// https://doc.rust-lang.org/book/ch08-02-strings.html#what-is-a-string

pub fn string_examples() {
    create_new_strings();
    update_strings();
    slice_strings();
}

fn create_new_strings() {
    let mut mutable_string = String::new();
    println!("strings: ");

    let initial_data = "Initial contents";
    let string_with_default_value = initial_data.to_string();

    // the method also works on a literal directly:
    let string_with_default_value = "Initial contents".to_string();

    // We can also use the function String::from to create a String from a string literal.
    let string_with_default_value = String::from("Initial contents");
}

fn update_strings() {
    let mut s = String::from("foo");
    s.push_str("bar"); // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter. For instance, if we want to be able to use `s2` after appending its contents to `s1`:

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("strings: s2 is {s2}"); // If the push_str method took ownership of s2, we wouldn’t be able to print its value on the last line.

    let mut s = String::from("lo");
    s.push('l');
}

fn concatenate_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // This looks pretty annoying to write, so let's use the `format!` macro instead:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}

fn slice_strings() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    for character in s.chars() {
        println!("strings: {character}");
    }

    for byte in s.bytes() {
        println!("strings: {byte}");
    }
}
