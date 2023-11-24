use std::cmp::Ordering::*;
use pancake::{Instruction::*, Register::*, Type::*};

#[test]
fn parsing_test() {
    let program = include_str!("test.txt");
    let parsed = pancake::parse_file(program).expect("parsing failed");
    assert_eq!(
        parsed,
        vec![
            PushInteger(100),
            PushFloat(-1.5e9),
            PushBoolean(true),
            PushCharacter(b'H'),
            PushCharacter(b'\x00'),
            PushRegister(X),
            Pop(Some(X)),
            Copy(X),
            Swap,
            Jump(39),
            Branch(0),
            Goto(X),
            Call(39),
            Return,
            Compare(Some(Equal)),
            Compare(None),
            Compare(Some(Less)),
            Compare(Some(Greater)),
            Add,
            Subtract,
            Multiply,
            Divide,
            Modulo,
            Negate(X),
            And,
            Or,
            Xor,
            Not(X),
            Shift,
            Rotate,
            Cast(Integer, X),
            Reinterpret(Integer, X),
            Input(Character, X),
            Read(Character, X),
            Output(X),
            Write(X),
            Random(Integer, X),
            Break,
            Drop(X)
        ]
    )
}
