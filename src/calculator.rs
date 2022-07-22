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
            Stack::push(&mut stack, result).unwrap();
        } else {
            let number: f64 = expression.parse::<f64>().unwrap();
            Stack::push(&mut stack, number).unwrap();
        }
    }

    if stack.top != 0 {
        Err("Error: Stack is not empty")
    } else {
        Ok(stack.array[0])
    }
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
    let mut wait = 0;       // wait for digits
    for (index, character) in instructions.chars().enumerate() {
        match (character, wait) {
            ('+' | '-', 0) => {             // positive or negative sign
                math_expre[counter] = &instructions[index - wait .. index + 1];
                wait = wait + 1;
            },
            ('+' | '-' | '*' | '/', _) => { // binary operations
                counter = counter + 1;
                math_expre[counter] = &instructions[index .. index + 1];
                counter = counter + 1;
                wait = 0;
            },
            (' ', _) => {},                 // ignore whitespaces
            _   => {                        // add a digit
                math_expre[counter] = &instructions[index - wait .. index + 1];
                wait = wait + 1;
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
