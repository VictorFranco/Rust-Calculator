//N=array size
const N:usize=10;
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
    let instructions=["1","2","+"];
    let mut num1:f64=0.0;
    let mut num2:f64=0.0;    
    for instruction in instructions.iter(){    
        let expression=*instruction;
        if expression=="+"||expression=="-"||expression=="*"||expression=="/" {
            pop(&mut stack,&mut num1);
            pop(&mut stack,&mut num2);
            let result=match &*expression{
                "+"=>num1+num2,
                "-"=>num1-num2,
                "*"=>num1*num2,
                "/"=>num1/num2,
                _=>0.0
            };
            push(&mut stack,result);
        }else{
            let number:f64=expression.parse::<f64>().unwrap();
            push(&mut stack,number);
        }
    }
    let mut math_expre=String::new();
    for instruction in instructions.iter(){
        math_expre.push_str(instruction);
        math_expre.push_str(" ");
    }
    println!(">>> {}",math_expre);
    println!("{:?}",stack.array[(stack.sp+1) as usize]);
}