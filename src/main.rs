mod stack;
mod calculator;
mod tree;

fn main() {
    println!("Rust Calculator\n");

    let instructions = ["1", "+", "2", "*", "3", "-", "1", "+", "1"];
    println!("Infix expression:");
    println!("{}", calculator::get_math_expre(&instructions));
    println!("\nPostfix expression:");
    calculator::postfix_expression(&instructions);
}
