use crate::List::{Cons, Nil};

fn main() {
    store_data_on_heap();
    enable_recursive_types();
}

fn store_data_on_heap() {
    let b = Box::new(5); // Box allow us to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

    // It should be noted that this is a very inefficient usage of Box, since if we wanted a single i32 stored, it'd make more sense to be on the stack for the majority of situations.
    println!("b = {}", b);
}

fn enable_recursive_types() {
    // A value of recursive type can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up. However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
}

// enum List {
//     Cons(i32, List),
//     Nil, // Note here that Nil is often used to denote the base case of a recursion, not the regular nil usage we use in rust to denote 0 or null/an absent value. We're just borrowing verbiage for this.

//          // also, in this state our List won't compile yet since our List type doesn't have a known size:

// //    Compiling cons-list v0.1.0 (file:///projects/cons-list)
// //    error[E0072]: recursive type `List` has infinite size
// //     --> src/main.rs:1:1
// //      |
// //    1 | enum List {
// //      | ^^^^^^^^^
// //    2 |     Cons(i32, List),
// //      |               ---- recursive without indirection
// //      |
// //    help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
// //      |
// //    2 |     Cons(i32, Box<List>),
// //      |               ++++    +

// }

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. This means we can put a Box<T> inside the Cons variant instead of another List value directly.
    Nil,
}
