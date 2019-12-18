extern crate sort;

use std::io;

fn main() {
    let mut inputs: Vec<i32> = Vec::new();
    println!("Please input numbers");
    for _i in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Filed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        inputs.push(input);
    }
    let sorted = sort::selection_sort::main(&inputs);
    println!("{:?}", sorted);
}
