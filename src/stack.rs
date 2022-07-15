//N = array size
const N:usize = 10;

pub struct Stack {
    pub array : [f64; N],
    pub sp    : i32
}

impl Stack {

    pub fn create_stack() -> Stack {
        Stack {
            array : [0.0; N],
            sp    : -1
        }
    }

    pub fn push(&mut self,value:f64) -> bool {
        if !self.is_full() {
            self.sp = self.sp + 1;
            self.array[self.sp as usize] = value;
            return true
        }
        return false
    }

    pub fn pop(&mut self,value:&mut f64) -> bool {
        if !self.is_empty() {
            *value  = self.array[self.sp as usize];
            self.array[self.sp as usize] = 0.0;
            self.sp = self.sp - 1;
            return true
        }
        return false
    }

    pub fn is_full(&self) -> bool {
        self.sp == (N as i32) - 1
    }

    pub fn is_empty(&self) -> bool {
        self.sp == -1
    }

}
