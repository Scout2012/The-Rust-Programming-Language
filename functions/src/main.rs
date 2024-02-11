fn main() {
    println!("Hello, world!");

    println!("Calling example");
    another_function();

    println!("Calling example");
    function_with_param(5);

    println!("Calling example");
    print_labeled_measurement(5, 'h');

    println!("Calling example");
    statement();

    println!("Calling example");
    expression();

    println!("Calling example");
    println!("The result of calling the five function is: {}", five());

    println!("Calling example");
    println!(
        "The result of calling plus_one function with 5: {}",
        plus_one(5)
    );
}

fn another_function() {
    println!("Another function.");
}

fn function_with_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statement() {
    // the assignment does not return a value, so this operation is a statement
    let _y = 6;
    // below cannot be performed as no value is returned from the y assignment
    // let x = (let y = 6);
    // in languages like JS and C, you can do something like
    // x = y = 6;
}

fn expression() {
    // the result of this block is bound to y, therefore the block is an expression
    let y = {
        let x = 3;
        x + 1 // no semi-colon so that this value is the resulting value from the expression
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
