mod stack;
mod calculator;

fn main(){
    println!("Calculator\n");

    let mut instructions = ["1","2","+"];
    let result = calculator::compute(&mut instructions);
    println!(">>> {}",calculator::get_math_expre(&mut instructions));
    println!("{}",result);

    let mut instructions = ["10","5","+","20","5","+","+"];
    let result = calculator::compute(&mut instructions);
    println!(">>> {}",calculator::get_math_expre(&mut instructions));
    println!("{}",result);
}
