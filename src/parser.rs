use crate::structures::*;
use std::{cmp::Ordering, collections::HashMap, str::FromStr};
impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "integer" => Type::Integer,
            "boolean" => Type::Boolean,
            "float" => Type::Float,
            "character" => Type::Character,
            _ => return Err(()),
        })
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "X" => Register::X,
            "Y" => Register::Y,
            _ => return Err(()),
        })
    }
}

// Macros for ergonomics within the match statement below

macro_rules! parse_register {
    ($name: ident) => {{
        let Ok(register) = Register::from_str($name) else {
            return Err(Error::InvalidInstruction);
        };
        register
    }};
}

macro_rules! parse_type {
    ($value: ident as char) => {{
        let bytes = $value.as_bytes();
        if bytes.len() != 3 {
            return Err(Error::InvalidInstruction);
        };
        if bytes[0] == bytes[2] && bytes[0] == b'\'' {
            bytes[1]
        } else if bytes[0] == b'#' {
            let string = [bytes[1], bytes[2]];
            let string = String::from_utf8_lossy(&string);
            let Ok(value) = u8::from_str_radix(&string, 16) else {
                return Err(Error::InvalidInstruction);
            };
            value
        } else {
                                        eprintln!("bad formatting");

            return Err(Error::InvalidInstruction);
        }
    }};
    ($value: ident as $ty: ty) => {{
        let Ok(value) = <$ty>::from_str($value) else {
            return Err(Error::InvalidInstruction);
        };
        value
    }};
}

macro_rules! next_word {
    ($words: ident => $target: ident) => {
        let Some($target) = $words.next() else {
            return Err(Error::InvalidInstruction);
        };
    };
}

impl Instruction {
    /// Parse an instruction.
    pub fn parse(mut line: &str, labels: &HashMap<&str, usize>) -> Result<Option<Instruction>, Error> {
        // Split the instruction into its parts
        line = line.trim();
        let mut words = line.split_ascii_whitespace();
        let Some(instruction_name) = words.next() else {
            // Just whitespace here, moving along
            return Ok(None);
        };
        // Giant match
        match instruction_name {
            "push" => {
                // Either a type or register
                next_word!(words => kind);
                next_word!(words => value);
                Ok(match kind {
                    "integer" => Some(Instruction::PushInteger(parse_type!(value as i64))),
                    "float" => Some(Instruction::PushFloat(parse_type!(value as f64))),
                    "boolean" => Some(Instruction::PushBoolean(parse_type!(value as bool))),
                    "character" => Some(Instruction::PushCharacter(parse_type!(value as char))),
                    "register" => Some(Instruction::PushRegister(parse_register!(value))),
                    _ => return Err(Error::InvalidInstruction),
                })
            }
            "pop" => {
                next_word!(words => register);
                Ok(Some(Instruction::Pop(if register == "_" {
                    None
                } else {
                    Some(parse_register!(register))
                })))
            }
            "copy" =>
                Ok(Some(Instruction::Copy)),
            "swap" =>
                Ok(Some(Instruction::Swap)),
            "length" => {
                next_word!(words => register);
                Ok(Some(Instruction::Length(parse_register!(register))))
            }
            "branch" => {
                next_word!(words => label_name);
                let Some(index) = labels.get(label_name) else {
                    return Err(Error::MissingLabel);
                };
                Ok(Some(Instruction::Branch(*index)))
            }
            "goto" => {
                next_word!(words => register);
                Ok(Some(Instruction::Goto(parse_register!(register))))
            }
            "call" => {
                next_word!(words => label_name);
                let Some(index) = labels.get(label_name) else {
                    return Err(Error::MissingLabel);
                };
                Ok(Some(Instruction::Call(*index)))
            }
            "return" => Ok(Some(Instruction::Return)),
            "compare" => {
                next_word!(words => comparison_mode);
                let comparison = match comparison_mode {
                    "equal" => Some(Ordering::Equal),
                    "unequal" => None,
                    "greater" => Some(Ordering::Greater),
                    "less" => Some(Ordering::Less),
                    _ => return Err(Error::InvalidInstruction),
                };
                Ok(Some(Instruction::Compare(comparison)))
            }
            "add" => Ok(Some(Instruction::Add)),
            "subtract" => Ok(Some(Instruction::Subtract)),
            "multiply" => Ok(Some(Instruction::Multiply)),
            "divide" => Ok(Some(Instruction::Divide)),
            "modulo" => Ok(Some(Instruction::Modulo)),
            "negate" => {
                next_word!(words => register);
                Ok(Some(Instruction::Negate(parse_register!(register))))
            }
            "and" => Ok(Some(Instruction::And)),
            "or" => Ok(Some(Instruction::Or)),
            "xor" => Ok(Some(Instruction::Xor)),
            "not" => {
                next_word!(words => register);
                Ok(Some(Instruction::Not(parse_register!(register))))
            }
            "shift" => Ok(Some(Instruction::Shift)),
            "rotate" => Ok(Some(Instruction::Rotate)),
            "cast" => {
                next_word!(words => ty);
                next_word!(words => register);
                let ty = Type::from_str(ty).map_err(|_| Error::InvalidInstruction)?;
                let register = parse_register!(register);
                Ok(Some(Instruction::Cast(ty, register)))
            }
            "reinterpret" => {
                next_word!(words => ty);
                next_word!(words => register);
                let ty = Type::from_str(ty).map_err(|_| Error::InvalidInstruction)?;
                if !matches!(ty, Type::Integer | Type::Float) {
                    return Err(Error::InvalidInstruction);
                }
                let register = parse_register!(register);
                Ok(Some(Instruction::Reinterpret(ty, register)))
            }
            "input" => {
                next_word!(words => ty);
                next_word!(words => register);
                let ty = Type::from_str(ty).map_err(|_| Error::InvalidInstruction)?;
                let register = parse_register!(register);
                Ok(Some(Instruction::Input(ty, register)))
            }
            "read" => {
                next_word!(words => ty);
                next_word!(words => register);
                let ty = Type::from_str(ty).map_err(|_| Error::InvalidInstruction)?;
                let register = parse_register!(register);
                Ok(Some(Instruction::Read(ty, register)))
            }
            "output" => {
                next_word!(words => register);
                let register = parse_register!(register);
                Ok(Some(Instruction::Output(register)))
            }
            "write" => {
                next_word!(words => register);
                let register = parse_register!(register);
                Ok(Some(Instruction::Write(register)))
            }
            "random" => {
                next_word!(words => ty);
                next_word!(words => register);
                let ty = Type::from_str(ty).map_err(|_| Error::InvalidInstruction)?;
                let register = parse_register!(register);
                Ok(Some(Instruction::Random(ty, register)))
            }
            "break" => Ok(Some(Instruction::Break)),
            "drop" => {
                next_word!(words => register);
                let register = parse_register!(register);
                Ok(Some(Instruction::Drop(register)))
            }
            "jump" => {
                next_word!(words => label_name);
                let Some(index) = labels.get(label_name) else {
                    return Err(Error::MissingLabel);
                };
                Ok(Some(Instruction::Jump(*index)))
            }
            // Comment
            line if line.starts_with('*') => Ok(None),
            _ => Err(Error::InvalidInstruction),
        }
    }
}

/// Parse an entire program from a string.
pub fn parse_file(file: impl AsRef<str>) -> Result<Vec<Instruction>, (usize, Error)> {
    let file = file.as_ref();
    let mut labels = HashMap::new();
    // Need to do two iterations, one for labels
    // A line doesn't necessarily map to an instruction,
    // so we can't enumerate for the jump indices
    let mut index = 0;
    for line in file.lines() {
        if !line.starts_with('\t') {
            labels.insert(line, index);
            continue;
        }
        let mut words = line.trim().split_ascii_whitespace();
        if let Some(word) = words.next() {
            if !word.starts_with('*') {
                // This line is an instruction
                index += 1;
            }
        }
    }
    let mut instructions = Vec::new();
    // Now for the actual parsing
    for (index, line) in file.lines().enumerate() {
        // We already handled labels
        if !line.starts_with('\t') {
            continue;
        }
        // Option<T> is an iterator!
        instructions.extend(Instruction::parse(line, &labels).map_err(|err| (index, err))?)
    }
    Ok(instructions)
}
