// The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

use std::iter;

fn main() {
    println!("Hello, world!");

    // In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // The iterator is stored in the v1_iter variable. Once we’ve created an iterator, we can use it in a variety of ways.

    for val in v1_iter {
        //  In languages that don’t have iterators provided by their standard libraries, you would likely write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.
        println!("Got: {}", val);
    }

    iterator_adaptor_map();
}

// All iterators implement a trait named Iterator that is defined in the standard library. The trait definition looks something like this:

pub trait IteratorExample {
    type Item;

    fn next(&mut self) -> Option<Self::Item>; // Notice this definition uses new syntax: `type Item` and `Self::Item, which define an associated type with this trait. On a shallow level, this code says that implementing the `IteratorExample` time also requires us to define an `Item` type, and this `Item` type is used in the return type of the `next` method.

    // The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time wrapped in `Some`, and returns `None` when iteration is over.

    // Check lib.rs to see us call the `next` method on iterators directly.
}

// Let's talk about Iterator adaptors. Iterator adaptors are methods defined on the Iterator trait that don't consume the iterator. They instead produce different iterators by changing some aspect of the original iterator:

fn iterator_adaptor_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
    // Please note that we need to consume the iterator here otherwise we'll get this warning:
    // = note: iterators are lazy and do nothing unless consumed
    // = note: `#[warn(unused_must_use)]` on by default

    // to fix this, we need to use the `collect` method:
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
