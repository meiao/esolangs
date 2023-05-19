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
    let mut blocks: Vec<StartBlockData> = vec![];
    let mut commands: Vec<Commands> = vec![];
    let mut line = 1;
    let mut col = 1;
    for char in source.chars() {
        match char {
            '>' => commands.push(Commands::IncDataPointer),
            '<' => commands.push(Commands::DecDataPointer),
            '+' => commands.push(Commands::IncData),
            '-' => commands.push(Commands::DecData),
            '.' => commands.push(Commands::Output),
            ',' => commands.push(Commands::Input),
            '[' => {
                blocks.push(StartBlockData {
                    start_block_instr: commands.len(),
                    line,
                    col,
                });
                // this is a temporary entry. The correct next_instr will be set when the block ends
                commands.push(Commands::StartBlock { next_instr: 0 })
            }
            ']' => match blocks.pop() {
                None => return Err(MismatchedClose { line, col }),
                Some(block_data) => {
                    commands[block_data.start_block_instr] = Commands::StartBlock {
                        next_instr: commands.len(),
                    };
                    commands.push(Commands::EndBlock {
                        next_instr: block_data.start_block_instr + 1,
                    });
                }
            },
            '\n' => {
                line = line + 1;
                col = 0;
            }
            _ => {}
        }
        col = col + 1;
    }

    if !blocks.is_empty() {
        let last_open = blocks.last().unwrap();
        return Err(MismatchedOpen {
            line: last_open.line,
            col: last_open.col,
        });
    }
    Ok(commands)
}

struct StartBlockData {
    start_block_instr: usize,
    line: u8,
    col: u8,
}

#[cfg(test)]
mod parser_test;
