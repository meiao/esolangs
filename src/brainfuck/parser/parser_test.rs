use BrainfuckError::MismatchedOpen;
use crate::brainfuck::BrainfuckError;
use crate::brainfuck::BrainfuckError::MismatchedClose;
use crate::brainfuck::parser::parse;
use crate::brainfuck::Commands::{
    DecData, DecDataPointer, EndBlock, IncData, IncDataPointer, StartBlock,
};

#[test]
fn parse_test() {
    let byte_code = parse(String::from("++[->+<]")).unwrap();
    assert_eq!(8, byte_code.len());
    let expected = vec![
        IncData,
        IncData,
        StartBlock {
            next_instr: Some(7),
        },
        DecData,
        IncDataPointer,
        IncData,
        DecDataPointer,
        EndBlock {
            next_instr: Some(2),
        },
    ];
    assert_eq!(expected.len(), byte_code.len());
    for i in 0..(expected.len() - 1) {
        assert_eq!(expected[i], byte_code[i])
    }
}

#[test]
fn mismatched_open() {
    let result = parse(String::from("[[]"));
    assert_eq!(result, Err(MismatchedOpen))
}

#[test]
fn mismatched_close() {
    let result = parse(String::from("[]]"));
    assert_eq!(result, Err(MismatchedClose))
}
