pub(crate) mod structures;
pub(crate) mod parser;

use std::io::{
    Read,
    Write
};

pub use structures::*;
pub use parser::parse_file;

impl Interpreter {
    fn register(&mut self, register: Register) -> &mut Option<Value> {
        match register {
            Register::X => &mut self.x,
            Register::Y => &mut self.y
        }
    }
    
    /// Execute an instruction in this interpreter.
    /// If successful, may return an index in the program to jump to.
    /// Returns an error if execution failed.
    pub fn execute(&mut self, instr: Instruction, input: impl Read, output: impl Write) -> Result<Option<usize>, Error> {
        match instr {
            // Pushing stuff
            Instruction::PushInteger(int) => 
                self.stack.push(Value::Integer(int)),
            Instruction::PushFloat(float) =>
                self.stack.push(Value::Float(float)),
            Instruction::PushBoolean(boolean) =>
                self.stack.push(Value::Boolean(boolean)),
            Instruction::PushCharacter(character) =>
                self.stack.push(Value::Character(character)),
            Instruction::PushRegister(reg) => {
                let Some(value) = self.register(reg).take() else {
                    return Err(Error::EmptyRegister)
                };
            },
            Instruction::Pop(_) => todo!(),
            Instruction::Copy(_) => todo!(),
            Instruction::Length(_) => todo!(),
            Instruction::Branch(_) => todo!(),
            Instruction::Compare(_) => todo!(),
            Instruction::Add => todo!(),
            Instruction::Subtract => todo!(),
            Instruction::Multiply => todo!(),
            Instruction::Divide => todo!(),
            Instruction::Modulo => todo!(),
            Instruction::Negate(_) => todo!(),
            Instruction::And => todo!(),
            Instruction::Or => todo!(),
            Instruction::Xor => todo!(),
            Instruction::Not(_) => todo!(),
            Instruction::Shift => todo!(),
            Instruction::Rotate => todo!(),
            Instruction::Cast(_, _) => todo!(),
            Instruction::Reinterpret(_, _) => todo!(),
            Instruction::Input(_, _) => todo!(),
            Instruction::Read(_, _) => todo!(),
            Instruction::Output(_) => todo!(),
            Instruction::Write(_) => todo!(),
            Instruction::Random(_, _) => todo!(),
            Instruction::Break => todo!(),
        }
        Ok(None)
    }
}