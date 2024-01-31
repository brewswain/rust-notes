// Traits are similar to a feature often called interfaces in other languages, although with some differences. A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

use std::fmt::{Debug, Display};

pub trait Summary {
    // this works just fine
    // fn summarize(&self) -> String;

    // We can specify a default string for the summarize method here if we wish
    // check the fn default_message_article() for implementation.
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // Uncomment if you don't want to use our default string in Summary

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn trait_examples() {
    // let's replicate this in our main.rs file. We'll have to use a use statement.
    let tweet = Tweet {
        username: String::from("heehee"),
        content: String::from("I smell like beef"),
        reply: false,
        retweet: false,
    };

    println!("traits_local: 1 new tweet: {}", tweet.summarize());

    default_message_article();
}

fn default_message_article() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    println!(
        "traits_local: New article available! {}",
        article.summarize()
    );
}

// Trait as parameters

// fn notify(item: &impl Summary) is syntactic sugar for a longer form like this:
// fn notify<T: Summary>(item: &T). Each has their own benefits--conciseness vs explicitness.
pub fn notify(item: &impl Summary) {
    // Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of NewsArticle or Tweet. Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.
    println!("Breaking news! {}", item.summarize());
}

// This approach is appropriate if we want this function to allow our different params to have diff types.
pub fn notify_with_more_complex_traits(item1: &impl Summary, item2: &impl Summary) {}

// This approach constrains the function so that the type of the value passed as params must be the same.
// notify_with_more_complex_traits<T: Summary>(item1: &T, item2: &T) {}

// Say we wanted notify to use display formatting as well as summarize on item: we specify in the notify definition that item must implement both Display and Summary. We can do so using the + syntax:
pub fn notify_with_multiple_trait_bounds(item: &(impl Summary + Display)) {}

// This also works with generics
pub fn notify_with_multiple_trait_bounds_generic<T: Summary + Display>(item: &T) {}

// Using too many trait bound traits can heavily impact readability. Take this function, for example:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0 // just returning 0 here to emulate successful fn call.
}

// Let's clean up the above, using a `where` statement:
fn some_function_concise<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// We can also use the impl Trait syntax in the return position

//By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type. In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t need to know that.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// However, you can only use impl Trait if you’re returning a single type.

// Using Trait bounds to conditionally implement methods:
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code:

// impl<T: Display> ToString for T {}

// Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:

// let s = 3.to_string();
