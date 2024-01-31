// A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.

// Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate.

// A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet
use std::{cmp::Ordering, io};
// use std::collections::*;

fn main() {
    println!("Hello, world!");
}

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // The add_to_waitlist function is defined in the same crate as eat_at_restaurant, which means we can use the crate keyword to start an absolute path. You can imagine a filesystem with the same structure: we’d specify the path /front_of_house/hosting/add_to_waitlist to run the add_to_waitlist program; using the crate name to start from the crate root is like using / to start from the filesystem root in your shell.

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // The second time we call add_to_waitlist in eat_at_restaurant, we use a relative path. The path starts with front_of_house, the name of the module defined at the same level of the module tree as eat_at_restaurant. Here the filesystem equivalent would be using the path front_of_house/hosting/add_to_waitlist. Starting with a module name means that the path is relative.
}

// Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.

// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#best-practices-for-packages-with-a-binary-and-a-library

fn deliver_order() {}

mod back_of_house {
    // We can also use pub to designate structs and enums as public, but there are a few details extra to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    } // If we make an enum public, all of its variants are then public. We only need the pub before the enum keyword

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            //  Because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast (in this case summer())
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root. From there, we look for deliver_order and find it. We used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    hosting::add_to_waitlist(); // We can do this now since our hosting module got added to scope via `use`

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// PLEASE NOTE: check lib.rs and front_of_house.rs for showing modules being used in multiple files.
