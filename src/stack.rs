//N = array size

#[derive(Debug)]
pub struct Stack<T, const N: usize> {
    pub array   : [T; N],
    pub top     : i32,
    pub default : T
}

impl<T: std::marker::Copy, const N: usize> Stack <T, N>{

    pub fn create_stack(default: T) -> Stack<T, N> {
        Stack {
            array    : [default; N],
            top      : -1,
            default  : default
        }
    }

    pub fn push(&mut self,value:T) -> Result<(), &str> {
        if !self.is_full() {
            self.top = self.top + 1;
            self.array[self.top as usize] = value;
            Ok(())
        } else {
            Err("Error: Stack overflow")
        }
    }

    pub fn pop(&mut self,value:&mut T) -> Result<(), &str> {
        if !self.is_empty() {
            *value  = self.array[self.top as usize];
            self.array[self.top as usize] = self.default;
            self.top = self.top - 1;
            Ok(())
        } else {
            Err("Error: Stack underflow")
        }
    }

    pub fn is_full(&self) -> bool {
        self.top == (N as i32) - 1
    }

    pub fn is_empty(&self) -> bool {
        self.top == -1
    }

}
