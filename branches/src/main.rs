fn main() {
    println!("Running if/else examples");
    if_else(3);
    if_else(7);
    if_else_chain(4);
    if_else_chain(3);
    if_else_chain(2);
    if_else_chain(5);
    if_else_var_assign(true);
    if_else_var_assign(false);
    loop_print(3);
    loop_return();
    loop_return_label();
    while_loop();
    while_array();
    for_array();
    for_range();
}

fn if_else(x: i32) {
    // the condition must result in a bool, not just a 0 or non-zero value
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_else_chain(x: i32) {
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_else_var_assign(condition: bool) {
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_print(x: i32) {
    let mut counter = 0;
    loop {
        if counter >= x {
            break;
        }

        counter += 1;

        if counter <= 1 {
            println!("We've completed {counter} loop");
        } else {
            println!("We've completed {counter} loops");
        };
    }
}

fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_return_label() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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

    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn while_array() {
    let numbers = [0, 1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 6 {
        println!("the value is: {}", numbers[index]);
        index += 1;
    }
}

fn for_array() {
    let numbers = [0, 1, 2, 3, 4, 5];

    for number in numbers {
        println!("the value is: {number}");
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("the value is: {number}");
    }
    println!("LIFTOFF!!!");
}
