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

fn main() -> Result<(),Error> {
    let mut stack : [i8 ; 64] = [0; 64];
    let mut memory : Vec<Memory256bits> = Vec::new();
    let mut pc : usize = 0;
    
    let instructions : &str = "6089605101";

    while pc*2 < instructions.len() {
        let opcode : u8 = instructions.as_bytes()[pc as usize] as u8;
        let opcode_result : Result<OpCode,Error> = OpCode::u8_to_opcode(opcode);
        match opcode_result {
            Ok(opcode) => {
                match opcode {
                    OpCode::PUSH => {
                        
                    },
                    OpCode::ADD => {
                    },
                }
            },
            Err( error)  => {
                return Err(Error::InvalidOpCode(opcode));
            }
        }
        pc += 1;
    } 
    instructions.parse::<i8>().unwrap();
}
