//N=array size
const N:usize=5;
struct Stack{
    array:[f64;N],
    sp:i32
}
fn create_stack()->Stack{
    Stack{
        array:[0.0;N],
        sp:(N as i32)-1
    }
}
fn push(stack:&mut Stack,value:f64)->bool{
    if !is_full(stack){        
        stack.array[stack.sp as usize]=value;
        stack.sp=stack.sp-1;
        return true
    }
    return false
}
fn pop(stack:&mut Stack,value:&mut f64)->bool{
    if !is_empty(stack){
        stack.sp=stack.sp+1;
        *value=stack.array[stack.sp as usize];
        stack.array[stack.sp as usize]=0.0;
        return true
    }
    return false
}
fn is_full(stack:&mut Stack)->bool{
    stack.sp==-1
}
fn is_empty(stack:&mut Stack)->bool{
    stack.sp==(N as i32)-1
}
fn main(){
    println!("Calculator");
    let mut stack:Stack=create_stack();
    let mut response:bool;
    response=push(&mut stack,53.0);
    println!("Push-->{}",response);
    response=push(&mut stack,30.0);
    println!("Push-->{}",response);
    let mut number:f64=0.0;
    response=pop(&mut stack,&mut number);
    println!("Number={:?} Pop-->{}",number,response);
    response=pop(&mut stack,&mut number);
    println!("Number={:?} Pop-->{}",number,response);
    
    println!("Array={:?}",stack.array);
}