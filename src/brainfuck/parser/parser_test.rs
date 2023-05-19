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

use crate::brainfuck::parser::parse;
use crate::brainfuck::BrainfuckError;
use crate::brainfuck::BrainfuckError::MismatchedClose;
use crate::brainfuck::Commands::{
    DecData, DecDataPointer, EndBlock, IncData, IncDataPointer, StartBlock,
};
use BrainfuckError::MismatchedOpen;

#[test]
fn parse_test() {
    let byte_code = parse(String::from("++[->+<]")).unwrap();
    assert_eq!(8, byte_code.len());
    let expected = vec![
        IncData,
        IncData,
        StartBlock { next_instr: 7 },
        DecData,
        IncDataPointer,
        IncData,
        DecDataPointer,
        EndBlock { next_instr: 2 },
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
    assert_eq!(result, Err(MismatchedClose { line: 1, col: 3 }))
}
