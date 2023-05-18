use crate::brainfuck::BrainfuckError::{DataOverflow, DataPointerNegative, DataPointerOverflow};
use crate::brainfuck::Commands::{
    DecData, DecDataPointer, EndBlock, IncData, IncDataPointer, StartBlock,
};
use crate::brainfuck::Machine;

#[test]
fn simple_test() {
    let commands = vec![
        IncData,
        IncDataPointer,
        DecData,
        DecData,
        DecDataPointer,
        IncData,
    ];
    let mut machine = Machine::create(commands);
    let result = machine.execute();
    assert_eq!(result, Ok(()));
    assert_eq!(machine.data[0], 2);
    assert_eq!(machine.data[1], -2);
}

#[test]
fn block_test() {
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
    let result = machine.execute();
    assert_eq!(result, Ok(()));
    assert_eq!(machine.data[0], 0);
    assert_eq!(machine.data[1], 2);
}

#[test]
fn data_pointer_negative() {
    let commands = vec![DecDataPointer];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataPointerNegative))
}

#[test]
fn data_pointer_overflow() {
    let commands = vec![IncData, StartBlock {next_instr: Some(3)}, IncDataPointer, IncData, EndBlock {next_instr: Some(1)}];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataPointerOverflow))
}

#[test]
fn positive_data_overflow() {
    let commands = vec![IncData, StartBlock {next_instr: Some(3)}, IncData, EndBlock {next_instr: Some(1)}];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataOverflow))
}

#[test]
fn negative_data_overflow() {
    let commands = vec![DecData, StartBlock {next_instr: Some(3)}, DecData, EndBlock {next_instr: Some(1)}];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataOverflow))
}
