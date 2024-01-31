// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

use std::collections::HashMap;

pub fn hashmap_examples() {
    create_hashmap();
    update_hashmap();
    update_value_based_on_old_value();
}

fn create_hashmap() {
    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("hashmaps: {key}: {value}");
    }

    // We can get a value out of the hash map by providing its key to the get method:

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    println!("hashmaps: {score}")
}

fn ownership_hashmap() {
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // / field_name and field_value are invalid at this point
}

fn update_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("hashmaps: {:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present

    scores.entry(String::from("Yellow")).or_insert(50); // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

    scores.entry(String::from("Blue")).or_insert(50);

    println!("hashmaps: {:?}", scores); //  The first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already. The second call to entry will not change the hash map because the Blue team already has the value 25.
}

fn update_value_based_on_old_value() {
    // Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, let's count how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("hashmaps: {:?}", map); // This code will print {"world": 2, "hello": 1, "wonderful": 1}.
}
