extern crate error_lib;
use error_lib::Error;

pub enum OpCode {
    PUSH = 0x60,
    ADD = 0x01
}

impl OpCode {
    pub fn u8_to_opcode(opcode : u8) -> Result<OpCode,Error>{
        match opcode  {
            0x60 => Ok(OpCode::PUSH),
            0x01 => Ok(OpCode::ADD),
            _ => Err(Error::InvalidOpCode(opcode)),
        }
    }
}
