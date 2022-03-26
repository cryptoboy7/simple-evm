extern crate opcode_lib;
extern crate error_lib;

use opcode_lib::opcode::OpCode;
use error_lib::Error;

fn get_last_stack_element(stack : &mut [i8;64]) -> i8{
    return stack[stack.len()-1];
}

fn get_stack_element(stack : &mut [i8;64], i :  usize ) -> Result<i8,Error>{
    if i > stack.len() {
        return Err(Error::IndexArrayOutOfBound(i));
    }else{
        return Ok(stack[i])
    }
}

struct Memory256bits {
    string256 : String
}

impl Memory256bits {
    fn new(s : String) -> Self {
        Self {
            string256 : s
        }
    }
}

fn main() {
    let mut stack : [i8 ; 64] = [0; 64];
    let mut memory : Vec<Memory256bits> = Vec::new();
    let mut pc : i8 = 0;
    
    let instructions : &str = "0x6089605101";

}
