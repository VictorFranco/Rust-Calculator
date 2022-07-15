use crate::stack::Stack as Stack;

pub fn compute(instructions: &[&str]) -> Result<f64, &'static str> {
    let mut stack : Stack = Stack::create_stack();
    let mut num1  : f64 = 0.0;
    let mut num2  : f64 = 0.0;
    let operators = ["+", "-", "*", "/"];

    for instruction in instructions.iter() {
        let expression = *instruction;
        if operators.contains(&expression) {
            pop(&mut stack, &mut num1);
            pop(&mut stack, &mut num2);
            let result = match expression {
                "+" => num2+num1,
                "-" => num2-num1,
                "*" => num2*num1,
                "/" => {
                    if num1 == 0.0 {
                        return Err("Error: Cannot divide by 0");
                    }
                    num2/num1
                },
                _   => 0.0
            };
            push(&mut stack, result);
        } else {
            let number: f64 = expression.parse::<f64>().unwrap();
            push(&mut stack, number);
        }
    }

    if stack.sp != 0 {
        Err("Error: Stack is not empty")
    }else {
        Ok(stack.array[0])
    }
}

pub fn get_math_expre(instructions: &[&str]) -> String {
    let mut math_expre = String::new();
    for instruction in instructions.iter() {
        math_expre.push_str(instruction);
        math_expre.push_str(" ");
    }
    math_expre
}

fn push(stack: &mut Stack, num: f64) {
    if !Stack::push(stack, num) {
        panic!("Error: Stack overflow");
    }
}

fn pop(stack: &mut Stack, num: &mut f64) {
    if !Stack::pop(stack, num) {
        panic!("Error: Stack underflow");
    }
}

pub fn operator_splits<'a>(instructions: &'a[&str], operator: &str) -> [&'a[&'a str]; 2] {
    let mut splits: [&'a[&'a str]; 2] = [&[""]; 2];
    for (index, instruction) in instructions.iter().enumerate() {
        if instruction == &operator {
            splits[0]  =  &instructions[..index];
            splits[1]  =  &instructions[index+1..];
        }
    }
    splits
}
