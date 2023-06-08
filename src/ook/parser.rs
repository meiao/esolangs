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

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::value;
use nom::error::ErrorKind;

use nom::Err::Failure;

use crate::brainfuck::Commands;

use crate::ook::parser::Token::{Exclamation, Period, Question};
use crate::ook::OokError;
use crate::ook::OokError::{OddTokenError, ParseError};
use nom::multi::many0;
use nom::IResult;
use Commands::{
    DecData, DecDataPointer, EndBlock, IncData, IncDataPointer, Input, Output, StartBlock,
};
use OokError::{MismatchedEnd, MismatchedStart};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Exclamation,
    Period,
    Question,
}

pub(super) fn parse(source: String) -> Result<Vec<Commands>, OokError> {
    let token_result = parse_program(source.as_str());
    if let Err(_) = token_result {
        return Err(ParseError);
    }
    let (_, mut tokens) = token_result.unwrap();
    if tokens.len() % 2 == 1 {
        return Err(OddTokenError);
    }

    let mut starts: Vec<usize> = Vec::new();
    let mut bf_tokens = Vec::new();
    tokens.reverse();
    while !tokens.is_empty() {
        let pair = (tokens.pop().unwrap(), tokens.pop().unwrap());
        let bf_token = match pair {
            (Exclamation, Exclamation) => Some(DecData),
            (Exclamation, Period) => Some(Output),
            (Exclamation, Question) => {
                starts.push(bf_tokens.len());
                Some(StartBlock { end_block_instr: 0 })
            }
            (Period, Exclamation) => Some(Input),
            (Period, Period) => Some(IncData),
            (Period, Question) => Some(IncDataPointer),
            (Question, Exclamation) => match starts.pop() {
                None => return Err(MismatchedEnd),
                Some(index) => {
                    bf_tokens[index] = StartBlock {
                        end_block_instr: bf_tokens.len(),
                    };
                    Some(EndBlock {
                        start_block_instr: index,
                    })
                }
            },
            (Question, Period) => Some(DecDataPointer),
            (Question, Question) => None,
        };
        if let Some(bf_token) = bf_token {
            bf_tokens.push(bf_token);
        }
    }
    if starts.len() > 0 {
        return Err(MismatchedStart);
    }
    Ok(bf_tokens)
}

fn parse_program(input: &str) -> IResult<&str, Vec<Token>> {
    let result = many0(parse_token)(input);
    if let Err(error) = result {
        return Err(error);
    }
    let (remaining, tokens) = result.unwrap();
    let (remaining, _) = multispace0::<&str, ()>(remaining).unwrap();
    if remaining.len() > 0 {
        return Err(Failure(nom::error::Error {
            input: remaining,
            code: ErrorKind::NonEmpty,
        }));
    }
    Ok((remaining, tokens))
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    let (input, _) = multispace0(input)?;
    alt((
        value(Exclamation, tag_no_case("ook!")),
        value(Period, tag_no_case("ook.")),
        value(Question, tag_no_case("ook?")),
    ))(input)
}

#[cfg(test)]
mod ook_parser_test;
