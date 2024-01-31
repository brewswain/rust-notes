// Remove Duplication by extracting a function
mod lifetimes;
mod traits;

use traits::{Summary, Tweet}; // see traits.rs

fn main() {
    naive_function();
    improved_function();
    split_function_without_generics();
    split_function_with_generics();

    // traits block
    traits::trait_examples();
    use_traits();

    // lifetimes block
    lifetimes::lifetime_examples();
}

// We'll build up this function to a way that works, but has duplicated code. We'll demonstration a solution for that without using generics(yet) in `improved_function()`. After that, we'll demonstrate a usecase where generics would be useful.
fn naive_function() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("generics: The largest number is {}", largest);

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("generics: The largest number is {}", largest);
}

// extracted logic
fn largest(list: &[i32]) -> &i32 // we accept any concrete slice of i32 values
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn improved_function() {
    // Much nicer!
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("generics: The largest number is {}", result);

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("generics: The largest number is {}", result);
}

// The above works great when we know that we'll only use one consistent data type--in this case, a vector of i32s. However, let's say we want the same functionality to find the largest item in 2 different types. In this case, assume the two types are a slice of `i32` values, and a slice of `char` values.

// First, let's have the code separated, then demonstrate the de-duplication strategy.

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn split_function_without_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("generics: The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("generics: The largest char is {}", result);
}

// Note the type here includes `std::cmp::PartialOrd`. This is a trait that ensures we only use types whose values can be ordered. s/o to rust's super clear compile errors!
fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn split_function_with_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_generic(&number_list);
    println!("generics: The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_generic(&char_list);
    println!("generics: The largest char is {}", result);
}

// see traits.rs
fn use_traits() {
    let tweet = Tweet {
        username: String::from("heehee"),
        content: String::from("I smell like beef"),
        reply: false,
        retweet: false,
    };

    println!("traits_import: 1 new tweet: {}", tweet.summarize());
}
