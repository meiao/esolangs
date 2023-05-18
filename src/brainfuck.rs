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
    StartBlock { next_instr: Option<usize> },
    EndBlock { next_instr: Option<usize> },
}

#[derive(Debug, PartialEq)]
pub enum BrainfuckError {
    MismatchedOpen,
    MismatchedClose,
    DataPointerNegative,
    DataPointerOverflow,
    DataOverflow,
    IoError,
}

impl BrainfuckError {
    fn to_error(&self) -> Error {
        match self {
            MismatchedOpen => Error {
                msg: String::from("Mismatched '['."),
                exit_code: 1,
            },
            MismatchedClose => Error {
                msg: String::from("Mismatched ']'."),
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

fn do_execute(commands: Vec<Commands>) -> Result<(), Error> {
    match Machine::create(commands).execute() {
        Ok(_) => Ok(()),
        Err(bf_error) => Err(bf_error.to_error()),
    }
}
