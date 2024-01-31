// Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:

// lifetimes are super good at preventing dangling references.

// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
pub fn lifetime_examples() {
    lifetime_annotations_in_function_signatures(("sds"), ("sdfsdfsd"));
}

// Uncomment to see error that we get
// fn prevent_dangling_reference_with_lifetime() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     //  the value that `r` is referring to has gone out of scope before we try to use it(&x). That being said, `r` is still valid for the outer scope. The compile error catches this error quite handily!
//     println!("r: {}", r);
// }
pub fn lifetime_annotations_in_function_signatures<'a>(x: &'a str, y: &'a str) -> &'a str {
    // To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.

    // We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the lifetime 'a and then add it to each reference.

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
