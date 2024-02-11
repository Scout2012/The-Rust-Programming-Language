use std::io;

fn let_mut() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn fp_types() {
    let _x = 2.0; // defaults to f64; double precision
    let _y: f32 = 3.0; // annotated to f32; single-precision
}

fn num_ops() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // -1 rather than -1.666... because no decimal provided

    // remainder
    let _remainder = 43 % 5;
}

fn bool_type() {
    let _t = true;

    let _f: bool = false;
}

fn char_type() {
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';
}

fn tuple_type() {
    // tuple is a compound type, as opposed to int, float, and char
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure the tuple into the x, y, and z variables
    let (_x, y, _z) = _tup;

    println!("The value of y is: {y}");

    // access tuple elements directly
    let _five_hundred = _tup.0;

    let _six_point_four = _tup.1;

    let _one = _tup.2;
}

fn array_type() {
    // the type annotation specs an array of 5 signed 32-bit integers
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // creates an array consisting of 3 five times.
    // equivalent to let b = [3, 3, 3, 3, 3];
    let _b = [3; 5];
}

fn array_indexing() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index of 0-4.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    let examples = [
        let_mut,
        shadowing,
        fp_types,
        num_ops,
        bool_type,
        char_type,
        tuple_type,
        array_type,
        array_indexing,
    ];

    for (index, example) in examples.iter().enumerate() {
        println!("Running example {index}");
        example();
    }
}
