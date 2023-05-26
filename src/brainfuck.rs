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

mod machine;
mod parser;

use crate::brainfuck::machine::Machine;
use crate::brainfuck::BrainfuckError::{
    DataPointerNegative, DataPointerOverflow, IoError, MismatchedClose, MismatchedOpen,
};
use crate::interpreter::Error;
use std::fs;
use BrainfuckError::DataOverflow;

#[derive(Debug, PartialEq)]
pub enum Commands {
    IncDataPointer,
    DecDataPointer,
    IncData,
    DecData,
    Output,
    Input,
    StartBlock { end_block_instr: usize },
    EndBlock { start_block_instr: usize },
}

#[derive(Debug, PartialEq)]
pub enum BrainfuckError {
    MismatchedOpen { line: u8, col: u8 },
    MismatchedClose { line: u8, col: u8 },
    DataPointerNegative,
    DataPointerOverflow,
    DataOverflow,
    IoError,
    InvalidInput,
}

impl BrainfuckError {
    fn to_error(&self) -> Error {
        match self {
            MismatchedOpen { line, col } => Error {
                msg: format!("Mismatched '[' at {}:{}.", line, col),
                exit_code: 1,
            },
            MismatchedClose { line, col } => Error {
                msg: format!("Mismatched ']' at {}:{}.", line, col),
                exit_code: 2,
            },
            DataPointerNegative => Error {
                msg: String::from("Tried to set data pointer to -1."),
                exit_code: 3,
            },
            DataPointerOverflow => Error {
                msg: String::from("Tried to set data pointer to 30000."),
                exit_code: 4,
            },
            DataOverflow => Error {
                msg: String::from("Data overflow."),
                exit_code: 5,
            },
            IoError => Error {
                msg: String::from("Error while reading source file."),
                exit_code: 6,
            },
            BrainfuckError::InvalidInput => Error {
                msg: String::from("Invalid input."),
                exit_code: 7,
            },
        }
    }
}

pub(super) fn execute(file_name: String) -> Result<(), Error> {
    let commands = match fs::read_to_string(file_name) {
        Ok(source) => parser::parse(source),
        Err(_) => Err(IoError),
    };

    match commands {
        Ok(commands) => do_execute(commands),
        Err(error) => Err(error.to_error()),
    }
}

pub(super) fn do_execute(commands: Vec<Commands>) -> Result<(), Error> {
    match Machine::create(commands).execute() {
        Ok(_) => Ok(()),
        Err(bf_error) => Err(bf_error.to_error()),
    }
}
