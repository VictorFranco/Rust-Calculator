mod stack;
mod calculator;
mod tree;

fn main() {
    println!("Rust Calculator\n");

    let instructions = ["1", "+", "2", "*", "3", "-", "1", "+", "1"];

    println!("Infix expression:");
    calculator::show_math_expre(&instructions);

    println!("\nPostfix expression:");
    let postfix = calculator::postfix_expression(&instructions);
    calculator::show_math_expre(&postfix.array);

    println!("\nResult:");
    println!("{}", calculator::compute(&postfix.array).unwrap());
}
