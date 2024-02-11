use std::{collections::HashMap, io};

// Generate the n-th fibonacci number
fn main() {
    let mut fib_mem: HashMap<u32, u64> = HashMap::new();

    println!("Which fibonacci number do you wish to calculate?");

    let n: u32 = loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read line.");
        match choice.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a positive number for N.");
                continue;
            }
        };
    };
    println!("the {n} fibonacci number is {}", fib(n, &mut fib_mem));
}

fn fib(n: u32, mem: &mut HashMap<u32, u64>) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        mem.insert(n, 1);
        return 1;
    }

    if !mem.contains_key(&n) {
        let val = fib(n - 1, mem) + fib(n - 2, mem);
        mem.insert(n, val);
    }

    return {
        match mem.get(&n) {
            Some(value) => *value,
            None => 1,
        }
    };
}
