fn main() {
    println!("Hello, world!");
    loop_call();
    loop_labels();
    while_call();
    for_call();
    for_in_loop();
    for_in_rev_loop();
}

fn loop_call() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop_call: The result is {result}");
}

fn loop_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("loop_labels: count = {count}");
        let mut remaining = 10;

        loop {
            println!("loop_labels: remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("loop_labels: End count = {count}");
}

fn while_call() {
    let mut number = 3;

    while number != 0 {
        println!("while_call: {number}");

        number -= 1;
    }

    println!("while_call: LIFTOFF!!!")
}

fn for_call() {
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("for_call:  the value is: {}", array[index]);

        index += 1;
    }
}

fn for_in_loop() {
    // the for in loop here is safer  than in for_call() here since it auto-determines the array length
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("for_in_loop: the value is: {element}");
    }
}

fn for_in_rev_loop() {
    for number in (1..4).rev() {
        println!("for_in_rev_loop: {number}");
    }
    println!("for_in_rev_loop: LIFTOFF!!!");
}
