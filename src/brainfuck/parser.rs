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

use crate::brainfuck::BrainfuckError::{MismatchedClose, MismatchedOpen};
use crate::brainfuck::{BrainfuckError, Commands};

pub(super) fn parse(source: String) -> Result<Vec<Commands>, BrainfuckError> {
    // TODO record line/char so a better error message can be provided for mismatched brackets
    let mut opens: Vec<usize> = vec![];
    let mut commands: Vec<Commands> = vec![];
    for char in source.chars() {
        match char {
            '>' => commands.push(Commands::IncDataPointer),
            '<' => commands.push(Commands::DecDataPointer),
            '+' => commands.push(Commands::IncData),
            '-' => commands.push(Commands::DecData),
            '.' => commands.push(Commands::Output),
            ',' => commands.push(Commands::Input),
            '[' => {
                opens.push(commands.len());
                commands.push(Commands::StartBlock { next_instr: None })
            }
            ']' => {
                match opens.pop() {
                    None => return Err(MismatchedClose),
                    Some(open_index) => {
                        commands.push(Commands::EndBlock {
                            next_instr: Some(open_index),
                        });
                        commands[open_index] = Commands::StartBlock {
                            next_instr: Some(commands.len() - 1),
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if !opens.is_empty() {
        return Err(MismatchedOpen);
    }
    Ok(commands)
}

#[cfg(test)]
mod parser_test;
