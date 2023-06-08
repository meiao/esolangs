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

use crate::interpreter::Error;
use clap::{Parser, Subcommand};
use Commands::{Brainfuck, Ook};

mod brainfuck;
mod interpreter;
mod ook;

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Brainfuck { source } => brainfuck::execute(source),
        Ook {source} => ook::execute(source),
    };
    match result {
        Err(error) => {
            exit(error);
        }
        Ok(_) => {}
    }
    println!();
}

fn exit(error: Error) {
    println!("{}", error.msg);
    std::process::exit(error.exit_code);
}

#[derive(Debug, Parser)]
#[command(name = "esolang")]
#[command(about = "A collection of esoteric languages interpreters.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Brainfuck { source: String },
    Ook { source: String},
}
