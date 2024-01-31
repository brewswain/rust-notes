// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type.

pub fn vector_examples() {
    let v: Vec<i32> = Vec::new(); // Vectors are implemented using generics.

    let mut mutable_vector = Vec::new();
    mutable_vector.push(5);
    mutable_vector.push(6);
    mutable_vector.push(7);
    mutable_vector.push(8);

    let vector_created_with_macro = vec![1, 2, 3, 4, 5];
    let third: &i32 = &vector_created_with_macro[2]; // Using & and [] gives us a reference to the element at the index value.
    println!("vectors: The third element is {third}");

    let third: Option<&i32> = vector_created_with_macro.get(2); // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
    match third {
        Some(third) => println!("vectors: The third element is matched to: {third}"),
        None => println!("vectors: There is no third element."),
    }

    let vector_created_with_macro2 = vec![1, 2, 3, 4, 5];

    // let does_not_exist_crash = &v[100]; // Will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
    let does_not_exist_resolve = v.get(100); // When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. Your code will then have logic to handle having either Some(&element) or None.

    // we can do for in loops:
    for item in &vector_created_with_macro2 {
        println!("vectors: {item}");
    }

    let mut mutable_vector_made_for_looping = vec![100, 32, 57];
    for item in &mut mutable_vector_made_for_looping {
        *item += 50 // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
    }

    println!("vectors: {:?}", mutable_vector_made_for_looping);

    use_enums_with_vectors();
}

// Vectors can only store values that are the same type. This can be inconvenient. Thankfully, we can define and use an enum to get one type to represent elements of different types.

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn use_enums_with_vectors() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("vectors: {:?}", row);

    // Like any other struct, a vector is freed when it goes out of scope.
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here. When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
}
