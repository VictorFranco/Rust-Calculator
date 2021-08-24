mod stack;
mod calculator;

fn main(){
    println!("Calculator\n");

    let instructions = ["1","2","+"];
    let result = calculator::compute(&instructions);
    println!(">>> {}",calculator::get_math_expre(&instructions));
    println!("{}",result.unwrap());

    let instructions = ["10","5","+","20","5","+","+"];
    let result = calculator::compute(&instructions);
    println!(">>> {}",calculator::get_math_expre(&instructions));
    println!("{}",result.unwrap());
}
