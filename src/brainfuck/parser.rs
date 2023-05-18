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
