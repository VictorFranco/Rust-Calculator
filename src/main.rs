mod stack;
mod calculator;
mod tree;
use tree::Tree as Tree;
fn main(){
    println!("Calculator\n");

    let mut instructions=["1","2","+"];
    let result=calculator::compute(&mut instructions); 
    println!(">>> {}",calculator::get_math_expre(&mut instructions));
    println!("{}",result);

    let mut instructions=["10","5","+","20","5","+","+"];
    let result=calculator::compute(&mut instructions); 
    println!(">>> {}",calculator::get_math_expre(& mut instructions));
    println!("{}",result);
    
    println!("\n");
    let mut t:Tree<String>=Tree::create_tree();
    Tree::insert_node(&mut t,String::from("60")); Tree::pre_order(&t); println!("");
    Tree::insert_node(&mut t,String::from("70")); Tree::pre_order(&t); println!("");
    Tree::insert_node(&mut t,String::from("65")); Tree::pre_order(&t); println!("");
    Tree::insert_node(&mut t,String::from("40")); Tree::pre_order(&t); println!("");
    Tree::insert_node(&mut t,String::from("50")); Tree::pre_order(&t); println!("");
    println!("\n");
    
    Tree::in_order(&t); println!("");
    Tree::post_order(&t);
}