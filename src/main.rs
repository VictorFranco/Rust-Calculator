mod stack;
mod calculator;
mod tree;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Rust Calculator\n");

    loop {
        let mut instructions = String::new();
        print!(">> ");
        stdout().flush().unwrap();                      // ensure print is emitted immediately
        stdin().read_line(&mut instructions).unwrap();  // get user input
        let instructions = instructions.trim();         // remove new line
        if  instructions == "" { continue }             // ignore input without instructions
        let instructions = calculator::get_math_array(&instructions);
        let postfix = calculator::postfix_expression(&instructions);
        print!(">> ");
        println!("{}", calculator::compute(&postfix.array).unwrap());
    }
}
