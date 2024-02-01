use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> { // old implementation changed to use our iterators.

    // Also, we're able to use mut keyword into the args param to make it mutable since we're taking ownership of args
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //     now that we implemented the Iterator trait by using args, we can call the next() method on it.

        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }

        // let query = args[1].clone(); // This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data. However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references.
        // let file_path = args[2].clone();

        // let's remove the clone() calls, but keep them commented out for reference instead of straight up deleting them from this project.

        args.next(); // We do this since the first value that gets returned by env::args is the name of the program, which we don't need.

        let query = match args.next() {
            // from here on, it's a pretty simple match pattern using next() to get to the value we put into the query field.
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).

    // Let's rewrite this code to use our iterator adaptors instead.
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     // Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines, which returns an iterator.
    //     if line.trim().contains(query.trim()) {
    //         results.push(line.trim());
    //     }
    // }

    // results

    // This is much more concise now, and also lets us avoid having a mutable intermediate results vector. This leans more into functional programming style, which tends to prefer a minimal amount of mutable state to make code clearer.
    contents
        .lines()
        .filter(|line| line.contains(query.trim()))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let's replicate the search() function's new design using iterators
    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.trim()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
