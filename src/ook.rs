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

use crate::brainfuck;
use crate::interpreter::Error;
use std::fs;
use OokError::{IoError, OddTokenError, ParseError};

mod parser;

pub(super) fn execute(file_name: String) -> Result<(), Error> {
    let commands = match fs::read_to_string(file_name) {
        Ok(source) => parser::parse(source),
        Err(_) => Err(IoError),
    };

    match commands {
        Ok(commands) => brainfuck::do_execute(commands),
        Err(error) => Err(Error::from(error)),
    }
}

#[derive(PartialEq, Debug)]
enum OokError {
    IoError,
    ParseError,
    OddTokenError,
    MismatchedEnd,
    MismatchedStart,
}

impl From<OokError> for Error {
    fn from(error: OokError) -> Self {
        match error {
            IoError => Error {
                msg: String::from("Error while reading the source file."),
                exit_code: 1,
            },
            ParseError => Error {
                msg: String::from("Error while parsing source file."),
                exit_code: 2,
            },
            OddTokenError => Error {
                msg: String::from("Odd number of tokens found."),
                exit_code: 3,
            },
            OokError::MismatchedEnd => Error {
                msg: String::from("Mismatched Ook! Ook?."),
                exit_code: 4,
            },
            OokError::MismatchedStart => Error {
                msg: String::from("Mismatched Ook? Ook!."),
                exit_code: 5,
            },
        }
    }
}
