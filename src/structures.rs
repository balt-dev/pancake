use std::fmt::{self, Display};

#[derive(Debug, Clone)]
/// A singular value.
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(u8),
}

#[derive(Debug, Copy, Clone)]
/// A type of a value.
pub enum Type {
    Integer,
    Float,
    Boolean,
    Character,
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Character => "character",
            Self::Float => "float"
        })
    }
}

#[derive(Debug, Copy, Clone)]
/// A register.
pub enum Register {
    X,
    Y,
}

#[derive(Debug, Clone)]
/// A singular instruction.
pub enum Instruction {
    /// Push an integer. 64-bit signed with two's complement wrapping.
    PushInteger(i64),
    /// Push a floating point. Double precision IEEE754.
    PushFloat(f64),
    /// Push a boolean.
    PushBoolean(bool),
    /// Push a character. Must be valid ASCII.
    PushCharacter(u8),
    /// Push the value in the register.
    /// Errors if the register is empty.
    PushRegister(Register),
    /// Pop a stack value into one of two registers.
    Pop(Register),
    /// Copies the value in a register and pushes it to the stack.
    Copy(Register),
    /// Puts the current length of the stack, as an integer,
    /// into the given register.
    Length(Register),

    /// Jumps to an instruction index if the value on the top of the stack is true,
    /// popping it. Errors if the value isn't a boolean.
    Branch(usize),

    /// Compares the values in X and Y, and pushes the boolean result
    /// to the stack. If equal or nonequal, the values may be of differing
    /// types, but if less or greater, an error is raised if they are.
    /// A boolean true is larger than a boolean false,
    /// and characters are compared by ASCII codepoint.
    /// Floats follow IEEE745 logic, meaning NaN != NaN.
    Compare(Option<std::cmp::Ordering>),
    /// Adds X and Y, and pushes the result to the stack.
    Add,
    /// Subtracts X from Y, and pushes the result to the stack.
    Subtract,
    /// Multiplies X and Y, and pushes the result to the stack.
    Multiply,
    /// Divides X and Y, and pushes the quotient to the stack.
    /// Raises an error on integer division by 0.
    Divide,
    /// Divides X and Y, and pushes the modulo
    /// (euclidean remainder) to the stack.
    /// Raises an error on integer division by 0.
    Modulo,
    /// Negates the value of the register, and leaves it in said register.
    Negate(Register),

    /// Takes the bitwise or logical AND of X and Y,
    /// and pushes the result to the stack.
    And,
    /// Takes the bitwise or logical OR of X and Y,
    /// and pushes the result to the stack.
    Or,
    /// Takes the bitwise or logical XOR of X and Y,
    /// and pushes the result to the stack.
    Xor,
    /// Takes the bitwise NOT of the register,
    /// and leaves it in said register.
    Not(Register),

    /// Shifts the integer in X to the right by
    /// the signed integral amount of bits in Y,
    /// discarding extra bits and padding with 0, and pushes the result.
    Shift,
    /// Shifts the integer in X to the right by
    /// the signed integral amount of bits in Y,
    /// wrapping extra bits to the other side, and pushes the result.
    Rotate,

    /// Attempts to cast the value in the register to the given type,
    /// leaving it in place. Pushes a boolean returning whether or not
    /// the cast successfully completed.
    Cast(Type, Register),
    /// Reinterprets the value in the register as the given type,
    /// without touching the bits.
    Reinterpret(Type, Register),

    /// Grabs a value from stdin as text, and puts it into this register.
    Input(Type, Register),
    /// Grabs a value from stdin as bytes, and puts it into this register.
    Read(Type, Register),
    /// Outputs the value in the register to stdout as text.
    Output(Register),
    /// Outputs the value in the register to stdout as bytes.
    Write(Register),

    /// Puts a random valid value of the specified type into the register.
    /// For floats, this may be any value in the half-open range [0.0, 1.0).
    Random(Type, Register),
    /// Immediately halts the program.
    Break,
}

#[derive(Debug, Clone, Default)]
/// An instance of an interpreter.
pub struct Interpreter {
    pub x: Option<Value>,
    pub y: Option<Value>,
    pub stack: Vec<Value>,
}

#[derive(Debug, Clone)]
/// A reason why execution of an instruction failed.
pub enum Error {
    /// Attempted to divide integral values by zero.
    DivideByZero,
    /// Failed to read from stdin.
    ReadFailed,
    /// Failed to write to stdout.
    WriteFailed,
    /// The type of the value was invalid for the instruction.
    InvalidType(Type),
    /// Mismatched types for comparison.
    MismatchedTypes(Type, Type),
    /// The stack was popped while empty.
    StackUnderflow,
    /// Encountered an invalid instruction while parsing.
    InvalidInstruction,
    /// One or more registers targeted were empty.
    EmptyRegister,
    /// One or more labels wasn't found.
    MissingLabel
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DivideByZero => write!(f, "attempted to divide by integral zero"),
            Error::ReadFailed => write!(f, "failed to read from stdin"),
            Error::WriteFailed => write!(f, "failed to write to stdout"),
            Error::InvalidType(ty) => write!(f, "failed to execute with invalid type {ty}"),
            Error::MismatchedTypes(ty1, ty2) => write!(
                f,
                "failed to compare value of type {ty1} with value of type {ty2}"
            ),
            Error::StackUnderflow => write!(f, "encountered a stack underflow"),
            Error::InvalidInstruction => write!(f, "encountered an invalid instruction"),
            Error::EmptyRegister => write!(f, "encountered an unexpected empty register"),
            Error::MissingLabel => write!(f, "could not find a matching label")
        }
    }
}
