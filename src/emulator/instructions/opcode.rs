use super::operand::Operands;

//TODO: remove this
#[allow(dead_code)]
pub struct Opcode {
    pub opcode_byte: u8,
    pub length: usize,
    pub cycle: Vec<i32>,
    pub mnemonic: String,
    pub operand1: Option<Operands>,
    pub operand2: Option<Operands>
} 

impl Opcode {

    pub fn opcode0(mnemonic: &str, opcode_byte: u8, length: usize, cycle: Vec<i32>) -> Self {
        Opcode {
            opcode_byte,
            length,
            cycle,
            mnemonic: String::from(mnemonic),
            operand1: None,
            operand2: None
        } 
    }

    pub fn opcode1(mnemonic: &str, opcode_byte: u8, length: usize, cycle: Vec<i32>, operand1: Operands) -> Self {
        Opcode {
            opcode_byte,
            length,
            cycle,
            mnemonic: String::from(mnemonic),
            operand1: Some(operand1),
            operand2: None
        } 
    }

    pub fn opcode2(mnemonic: &str, opcode_byte: u8, length: usize, cycle: Vec<i32>, operand1: Operands, operand2: Operands) -> Self {
        Opcode {
            opcode_byte,
            length,
            cycle,
            mnemonic: String::from(mnemonic),
            operand1: Some(operand1),
            operand2: Some(operand2),
        } 
    }

}
