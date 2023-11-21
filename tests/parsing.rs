use pancake::Instruction;

#[test]
fn parsing_test() {
    let program = r#"
        label LABEL
        push boolean true
        branch LABEL
    "#.trim();
    let parsed = pancake::parse_file(program).expect("parsing failed");
    assert_eq!(parsed, vec![Instruction::PushBoolean(true), Instruction::Branch(0)])
}