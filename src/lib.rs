extern crate core;

pub(crate) mod parser;
pub(crate) mod structures;

use rand::Rng;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

pub use parser::parse_file;
pub use structures::*;

// Macros for ergonomics inside the function
macro_rules! take {
    ($self: ident . $reg: ident) => {{
        let Some(value) = $self.register($reg).take() else {
            return Err(Error::EmptyRegister);
        };
        value
    }};
}

macro_rules! reg {
    ($self: ident . $reg: ident) => {{
        let Some(value) = $self.register($reg) else {
            return Err(Error::EmptyRegister);
        };
        value
    }};
}

macro_rules! typed {
    ($value: expr => $ty: ident) => {{
        let value = $value;
        let Value::$ty(typed_value) = value else {
            return Err(Error::InvalidType(value.get_type()));
        };
        typed_value
    }};
}

macro_rules! math {
    ($self: ident, $for_int: ident, $for_float: tt) => {{
        let lhs = take!($self.X);
        let rhs = take!($self.Y);
        match (lhs, rhs) {
            (Value::Integer(l), Value::Integer(r)) =>
                $self.stack.push((l.$for_int(r)).into()),
            (Value::Float(l), Value::Float(r)) =>
                $self.stack.push((l $for_float r).into()),
            (l, r) =>
                return Err(Error::MismatchedTypes(l.get_type(), r.get_type()))
        }
    }};
}

macro_rules! logic {
    ($self: ident, $operand: tt) => {{
        let lhs = take!($self.X);
        let rhs = take!($self.Y);
        match (lhs, rhs) {
            (Value::Integer(l), Value::Integer(r)) =>
                $self.stack.push((l $operand r).into()),
            (Value::Boolean(l), Value::Boolean(r)) =>
                $self.stack.push((l $operand r).into()),
            (l, r) =>
                return Err(Error::MismatchedTypes(l.get_type(), r.get_type()))
        }
    }};
}

// Interpreter implementation
impl<R: Rng> Interpreter<R> {
    fn register(&mut self, register: Register) -> &mut Option<Value> {
        match register {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
        }
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    /// Execute an instruction in this interpreter.
    /// If successful, may return an index in the program to jump to.
    /// Returns an error if execution failed.
    pub fn execute(
        &mut self,
        index: usize,
        instr: Instruction,
        mut input: impl Read,
        mut output: impl Write,
    ) -> Result<Option<usize>, Error> {
        use Register::*;
        match instr {
            // Pushing stuff
            Instruction::PushInteger(int) => self.stack.push(Value::Integer(int)),
            Instruction::PushFloat(float) => self.stack.push(Value::Float(float)),
            Instruction::PushBoolean(boolean) => self.stack.push(Value::Boolean(boolean)),
            Instruction::PushCharacter(character) => self.stack.push(Value::Character(character)),
            Instruction::PushRegister(reg) => {
                let value = take!(self.reg);
                self.stack.push(value);
            }
            Instruction::Pop(reg) => {
                let value = self.pop()?;
                let Some(reg) = reg else {return Ok(None)};
                let register = self.register(reg);
                let _ = register.insert(value);
            }
            Instruction::Copy =>
                self.y = Some(reg!(self.X).clone()),
            Instruction::Swap =>  {
                // Can't use reg! macro due to borrow checker pains
                let Some(x) = &mut self.x else {
                    return Err(Error::EmptyRegister);
                };
                let Some(y) = &mut self.y else {
                    return Err(Error::EmptyRegister);
                };
                std::mem::swap(x, y);
            },
            Instruction::Length(reg) => {
            // Even on 64-bit, stack length has to be at most i64::MAX, so this is fine
                *self.register(reg) = Some((self.stack.len() as i64).into())
            }
            // Moving around the instruction pointer
            Instruction::Jump(to) => return Ok(Some(to)),
            Instruction::Branch(to) => {
                if typed!(self.pop()? => Boolean) {
                    return Ok(Some(to));
                }
            }
            Instruction::Goto(reg) => {
                let index = typed!(take!(self.reg) => Integer);
                if index < 0 {
                    return Ok(Some(usize::MAX));
                }
                return Ok(Some(index as usize));
            }
            // Call and return
            Instruction::Call(to) => {
                self.stack.push((index as i64).into());
                return Ok(Some(to));
            }
            Instruction::Return => {
                let popped = self.pop()?;
                let Value::Integer(to) = popped else {
                    return Err(Error::InvalidType(popped.get_type()));
                };
                return Ok(Some(to as usize));
            }
            // Math!
            Instruction::Compare(kind) => {
                let lhs = take!(self.X);
                let rhs = take!(self.Y);
                // Checking if two values of different types is equal is fine
                let result = if let Some(comparison) = kind {
                    // This match statement doesn't work in the left hand side of the below
                    // equality check for some reason
                    let compared = match (lhs, rhs) {
                        (Value::Integer(i), Value::Float(f)) => (i as f64).partial_cmp(&f),
                        (Value::Float(f), Value::Integer(i)) => f.partial_cmp(&(i as f64)),
                        (l, r) => l.partial_cmp(&r),
                    };
                    compared == Some(comparison)
                } else {
                    lhs != rhs
                };
                self.stack.push(result.into())
            }
            Instruction::Add => math!(self, wrapping_add, +),
            Instruction::Subtract => math!(self, wrapping_sub, -),
            Instruction::Multiply => math!(self, wrapping_mul, *),
            Instruction::Divide => {
                let lhs = take!(self.X);
                let rhs = take!(self.Y);
                match (lhs, rhs) {
                    (Value::Integer(l), Value::Integer(r)) => {
                        if r == 0 {
                            return Err(Error::DivideByZero);
                        }
                        self.stack.push(l.wrapping_div(r).into())
                    }
                    (Value::Float(l), Value::Float(r)) => self.stack.push((l / r).into()),
                    (l, r) => return Err(Error::MismatchedTypes(l.get_type(), r.get_type())),
                }
            }
            Instruction::Modulo => {
                let lhs = take!(self.X);
                let rhs = take!(self.Y);
                match (lhs, rhs) {
                    (Value::Integer(l), Value::Integer(r)) => {
                        if r == 0 {
                            return Err(Error::DivideByZero);
                        }
                        self.stack.push(l.wrapping_rem(r).into())
                    }
                    (Value::Float(l), Value::Float(r)) => self.stack.push((l % r).into()),
                    (l, r) => return Err(Error::MismatchedTypes(l.get_type(), r.get_type())),
                }
            }
            Instruction::Negate(reg) => {
                let value = reg!(self.reg);
                match value {
                    Value::Integer(i) => *i = -*i,
                    Value::Float(f) => *f = -*f,
                    v => return Err(Error::InvalidType(v.get_type())),
                }
            }
            Instruction::And => logic!(self, &),
            Instruction::Or => logic!(self, |),
            Instruction::Xor => logic!(self, ^),
            Instruction::Not(reg) => {
                let value = reg!(self.reg);
                match value {
                    Value::Integer(i) => *i = !*i,
                    Value::Boolean(b) => *b = !*b,
                    v => return Err(Error::InvalidType(v.get_type())),
                }
            }
            Instruction::Shift => {
                let lhs = typed!(take!(self.X) => Integer);
                let rhs = typed!(take!(self.Y) => Integer);
                let direction = rhs < 0; // Left if below 0
                let rhs = rhs.unsigned_abs() as u32;
                self.stack.push(
                    if direction {
                        lhs.wrapping_shl(rhs)
                    } else {
                        // Logical bit shift
                        (lhs as u64).wrapping_shr(rhs) as i64
                    }
                    .into(),
                )
            }
            Instruction::Rotate => {
                let lhs = typed!(take!(self.X) => Integer);
                let rhs = typed!(take!(self.Y) => Integer);
                let direction = rhs < 0; // Left if below 0
                let rhs = rhs.unsigned_abs() as u32;
                self.stack.push(
                    if direction {
                        lhs.rotate_left(rhs)
                    } else {
                        (lhs as u64).rotate_right(rhs) as i64
                    }
                    .into(),
                )
            }
            Instruction::Cast(ty, reg) => {
                let value = reg!(self.reg);
                if value.get_type() == ty {
                    self.stack.push(true.into());
                    return Ok(None); // early return
                }
                *value = match (value.clone(), ty) {
                    (Value::Boolean(value), Type::Integer) => (value as i64).into(),
                    (Value::Boolean(value), Type::Character) => (value as u8).into(),
                    (Value::Integer(value), Type::Boolean) => (value != 0).into(),
                    (Value::Integer(value), Type::Float) => (value as f64).into(),
                    (Value::Integer(value), Type::Character) => (value as u8).into(),
                    (Value::Float(value), Type::Integer) => (value as i64).into(),
                    (Value::Character(value), Type::Boolean) => (value != 0).into(),
                    (Value::Character(value), Type::Integer) => (value as i64).into(),
                    (v, ty) => return Err(Error::MismatchedTypes(v.get_type(), ty)),
                };
            }
            Instruction::Reinterpret(ty, reg) => {
                let value = reg!(self.reg);
                if value.get_type() == ty {
                    self.stack.push(true.into());
                    return Ok(None); // early return
                }
                *value = match (value.clone(), ty) {
                    (Value::Boolean(value), Type::Integer) => (value as i64).into(),
                    (Value::Boolean(value), Type::Character) => (value as u8).into(),
                    (Value::Integer(value), Type::Float) => f64::from_bits(value as u64).into(),
                    (Value::Float(float), Type::Integer) => (float.to_bits() as i64).into(),
                    (Value::Character(char), Type::Integer) => (char as i64).into(),
                    (v, ty) => return Err(Error::MismatchedTypes(v.get_type(), ty)),
                }
            }
            Instruction::Input(ty, reg) => {
                let mut buffered = BufReader::new(input);
                let mut parsed = None;
                let mut string = String::new();
                while parsed.is_none() {
                    buffered.read_line(&mut string)?;
                    let trimmed = string.trim();
                    // Mapping with Value::from works, .map can take more than closures
                    parsed = match ty {
                        Type::Integer => i64::from_str(trimmed).ok().map(Value::from),
                        Type::Float => f64::from_str(trimmed).ok().map(Value::from),
                        Type::Boolean => bool::from_str(trimmed).ok().map(Value::from),
                        // Grab the first byte of the string and treat that as the character
                        Type::Character => trimmed.bytes().next().map(Value::from),
                    }
                }
                *self.register(reg) = Some(parsed.unwrap())
            }
            Instruction::Read(ty, reg) => {
                let mut parsed = None;
                while parsed.is_none() {
                    // Mapping with Value::from works, .map can take more than closures
                    parsed = match ty {
                        Type::Integer => {
                            let mut buf = [0; 8];
                            input.read_exact(&mut buf)?;
                            Some(i64::from_be_bytes(buf).into())
                        }
                        Type::Float => {
                            let mut buf = [0; 8];
                            input.read_exact(&mut buf)?;
                            Some(f64::from_be_bytes(buf).into())
                        }
                        Type::Boolean => {
                            let mut buf = [0];
                            input.read_exact(&mut buf)?;
                            Some((buf[0] != 0).into())
                        }
                        Type::Character => {
                            let mut buf = [0];
                            input.read_exact(&mut buf)?;
                            Some(buf[0].into())
                        }
                    }
                }
                *self.register(reg) = parsed;
            }
            Instruction::Output(reg) => {
                let value = take!(self.reg);
                write!(output, "{value}").map_err(|_| Error::WriteFailed)?;
            }
            Instruction::Write(reg) => {
                let value = take!(self.reg);
                // These all have the same type so we can map err outside
                match value {
                    Value::Integer(i) => output.write_all(&i.to_be_bytes()),
                    Value::Float(f) => output.write_all(&f.to_be_bytes()),
                    Value::Boolean(b) => output.write_all(&[b as u8]),
                    Value::Character(c) => output.write_all(&[c]),
                }
                .map_err(|_| Error::WriteFailed)?;
            }
            Instruction::Random(ty, reg) => {
                *self.register(reg) = Some(match ty {
                    Type::Integer => self.rng.gen::<i64>().into(),
                    Type::Float => self.rng.gen::<f64>().into(),
                    Type::Boolean => self.rng.gen::<bool>().into(),
                    Type::Character => self.rng.gen::<u8>().into(),
                })
            }
            Instruction::Break => return Ok(Some(usize::MAX)),
            Instruction::Drop(reg) => *self.register(reg) = None,
        }
        Ok(None)
    }
}
