// The issue with using tuples to return our values as we've shown in our ownership section is that we have to return the String to the calling function so that we can still use the String after the call to `calculate_length`, since the string was moved into it.

// Instead of that, let's just provide a reference to the String value. A reference is like a pointer in that it's an address we can follow to access the data stored at that address. However, unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

//
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    modify_reference_success();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope. Because it doesn't have ownership of what it refers too, however, it isn't dropped.

// something interesting to note, is that if we try to modify any values we borrow, it won't work and we'll get a compile error:

fn modify_reference_fail() {
    let s = String::from("modify_reference_fail: hello");

    change_fail(&s);
}

fn change_fail(string_param: &String) {
    string_param.push_str(", world"); // this code will cause a compile error, so comment it out if u want to run the program
}

fn modify_reference_success() {
    let mut s = String::from("modify_reference_success: hello");

    change_success(&mut s);

    println!("{}", s)
}

fn change_success(string_param: &mut String) {
    string_param.push_str(", world"); // this code will cause a compile error, so comment it out if u want to run the program
}
