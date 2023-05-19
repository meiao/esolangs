/*
 *     Copyright (C) 2023  meiao
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 */

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
        StartBlock { next_instr: 7 },
        DecData,
        IncDataPointer,
        IncData,
        DecDataPointer,
        EndBlock { next_instr: 2 },
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
    let commands = vec![
        IncData,
        StartBlock { next_instr: 3 },
        IncDataPointer,
        IncData,
        EndBlock { next_instr: 1 },
    ];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataPointerOverflow))
}

#[test]
fn positive_data_overflow() {
    let commands = vec![
        IncData,
        StartBlock { next_instr: 3 },
        IncData,
        EndBlock { next_instr: 1 },
    ];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataOverflow))
}

#[test]
fn negative_data_overflow() {
    let commands = vec![
        DecData,
        StartBlock { next_instr: 3 },
        DecData,
        EndBlock { next_instr: 1 },
    ];
    let result = Machine::create(commands).execute();
    assert_eq!(result, Err(DataOverflow))
}
