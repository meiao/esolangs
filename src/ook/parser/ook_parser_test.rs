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

use crate::brainfuck::Commands::*;
use crate::ook::parser::{parse, parse_program};
use crate::ook::parser::{Exclamation, Period, Question};
use crate::ook::OokError;
use nom::error::ErrorKind::NonEmpty;
use nom::Err::Failure;
use OokError::OddTokenError;

#[test]
fn parse_tokens_test() {
    let (remaining, tokens) = parse_program("Ook.  \tOOK!  ook?\nook.    ").unwrap();
    assert_eq!(
        tokens,
        vec![Period, Exclamation, Question, Period],
        "Remaining: {remaining}"
    )
}

#[test]
fn parse_tokens_bad_test() {
    let Err(error) = parse_program("ook?nook.") else { panic!("An error was expected.") };
    let error = match error {
        Failure(error) => error,
        _ => panic!("An error was expected."),
    };
    let nom::error::Error { input, code } = error;
    assert_eq!(code, NonEmpty);
    assert_eq!(input, "nook.");
}

#[test]
fn parse_odd_token_count_test() {
    let Err(error) = parse(String::from("Ook.OOk.ook.")) else { panic!("An error was expected.") };
    assert_eq!(OddTokenError, error);
}

#[test]
fn parse_program_test() {
    let Ok(bf_tokens) = parse(String::from("Ook.Ook.Ook.Ook.Ook!Ook?Ook!Ook!Ook.Ook?Ook.Ook.Ook?Ook.Ook?Ook!Ook.Ook!Ook!Ook."))
        else { panic!("An error was expected.") };
    let expected = vec![
        IncData,
        IncData,
        StartBlock { end_block_instr: 7 },
        DecData,
        IncDataPointer,
        IncData,
        DecDataPointer,
        EndBlock {
            start_block_instr: 2,
        },
        Input,
        Output,
    ];
    assert_eq!(expected, bf_tokens);
}
