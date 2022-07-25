use crate::stack::Stack as Stack;
use crate::tree::Tree as Tree;
use crate::tree::Node as Node;
//N = array size
const N: usize = 200;

pub fn compute(instructions: &[&str]) -> Result<f64, &'static str> {
    let mut stack : Stack<f64, 20> = Stack::create_stack(0.0);
    let mut num1  : f64 = 0.0;
    let mut num2  : f64 = 0.0;
    let operators = ["+", "-", "*", "/"];

    for instruction in instructions.iter() {
        let expression = *instruction;
        if expression == "" { break }

        if operators.contains(&expression) {
            Stack::pop(&mut stack, &mut num1).unwrap();
            Stack::pop(&mut stack, &mut num2).unwrap();
            let result = match expression {
                "+" => num2 + num1,
                "-" => num2 - num1,
                "*" => num2 * num1,
                "/" => {
                    if num1 == 0.0 {
                        return Err("Error: Cannot divide by 0");
                    }
                    num2 / num1
                },
                _   => 0.0
            };
            Stack::push(&mut stack, result).unwrap();
        } else {
            let number: f64 = parse(&expression);
            Stack::push(&mut stack, number).unwrap();
        }
    }

    if stack.top != 0 {
        Err("Error: Stack is not empty")
    } else {
        Ok(stack.array[0])
    }
}

pub fn parse(expression: &str) -> f64 {     // parse numbers with multiple signs
    let mut sign = 1.0;
    let mut start_number = 0;
    for (index, character) in expression.chars().enumerate() {
        match character {
            '+' | ' ' => {},                // positive sign does not affect the number sign
            '-' => {sign = sign * -1.0},    // change the number sign
            _   => {                        // the character is a digit
                start_number = index;       // save the number position
                break;
            }
        }
    }
    let number: f64 = (&expression[start_number ..]).parse::<f64>().unwrap();
    sign * number
}

#[allow(dead_code)]
pub fn show_math_expre(instructions: &[&str]) {
    let mut math_expre = String::new();
    for &instruction in instructions.iter() {
        if instruction == "" { break }
        math_expre.push_str(instruction);
        math_expre.push_str(" ");
    }
    println!("{}", math_expre);
}

pub fn get_math_array<'a>(instructions: &'a str) -> [&'a str; N] {
    let mut math_expre: [&'a str; N] = [""; N];
    let mut counter = 0;    // array index
    let mut wait = 0;       // wait for number
    let mut operation = false;
    for (index, character) in instructions.chars().enumerate() {
        match (character, operation) {
            ('+' | '-', false) => {             // character is positive or negative sign
                math_expre[counter] = &instructions[index - wait .. index + 1];
                wait = wait + 1;
            },
            ('+' | '-' | '*' | '/', true) => {  // character is a binary operation
                counter = counter + 1;
                math_expre[counter] = &instructions[index .. index + 1];
                counter = counter + 1;
                wait = 0;
                operation = false;          // the next character is a sign or a digit
            },
            (' ', _) => {wait = wait + 1},  // ignore whitespaces
            _   => {                        // character is a digit
                math_expre[counter] = &instructions[index - wait .. index + 1];
                wait  = wait + 1;
                operation = true;           // the next character is an operation or a digit
            }
        }
    }
    math_expre
}

pub fn split_by_operator<'a>(instructions: &'a[&str], operator: &str) -> [&'a[&'a str]; 2] {
    let mut splits: [&'a[&'a str]; 2] = [&[""]; 2];
    for (index, instruction) in instructions.iter().enumerate() {
        if instruction == &operator {
            splits[0]  =  &instructions[.. index];
            splits[1]  =  &instructions[index + 1 ..];
        }
    }
    splits
}

pub fn postfix_expression<'a>(instructions: &'a[&str]) -> Stack<&'a str, N>{
    let mut tree: Tree<&str> = Tree::create_tree();
    let root_node = expression_tree(&instructions);
    let mut stack: Stack<&str, N> = Stack::create_stack("");
    Tree::insert_first(&mut tree, root_node);
    Tree::post_order(&tree, &mut stack);
    stack
}

pub fn expression_tree<'a>(instructions: &'a[&str]) -> Option<Box<Node<&'a str>>> {
    let order = [["+", "-"], ["*", "/"]];    // order of operations
    for group in order.iter() {
        // get last operator by order
        let last_operator = instructions.iter().rposition(|&instruction| group.contains(&instruction));
        match last_operator {
            Some(index) => {
                let operator: &'a str = instructions[index];
                let split = split_by_operator(&instructions, operator);
                let mut operator_node = Node::create_node(operator); // internal node
                operator_node.left    = expression_tree(split[0]);   // generate sub-trees
                operator_node.right   = expression_tree(split[1]);
                return Some(Box::new(operator_node));
            },
            None => {}
        }
    }

    let operand_node: Node<&'a str> = Node::create_node(instructions[0]); // leaf node
    return Some(Box::new(operand_node)); // return smart pointer
}
