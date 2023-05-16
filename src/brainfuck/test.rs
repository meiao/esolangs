use crate::brainfuck::parse;
use crate::brainfuck::Commands::{
    DecData, DecDataPointer, EndBlock, IncData, IncDataPointer, StartBlock,
};
use crate::brainfuck::Machine;

#[test]
fn first_test() {
    let commands = vec![
        IncData,
        IncDataPointer,
        DecData,
        DecData,
        DecDataPointer,
        IncData,
    ];
    let mut machine = Machine::create(commands);
    machine.execute();
    assert_eq!(machine.data[0], 2);
    assert_eq!(machine.data[1], -2);
}

#[test]
fn addition_test() {
    let commands = vec![
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
    let mut machine = Machine::create(commands);
    machine.execute();
    assert_eq!(machine.data[0], 0);
    assert_eq!(machine.data[1], 2);
}

#[test]
fn test_parse() {
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
    for i in 0..(expected.len() - 1) {
        assert_eq!(expected[i], byte_code[i])
    }
}
