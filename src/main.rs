use Commands::Brainfuck;
use crate::interpreter::Error;
use clap::{Parser, Subcommand};

mod brainfuck;
mod interpreter;

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Brainfuck { source } => {
            brainfuck::execute(source)
        }
    };
    match result {
        Err(error) => {
            exit(error);
        }
        Ok(_) => {}
    }
}

fn exit(error: Error) {
    print!("{}", error.msg);
    std::process::exit(error.exit_code);
}

#[derive(Debug, Parser)]
#[command(name = "esolang")]
#[command(about = "A collection of esoteric languages interpreters.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Brainfuck {
        source: String
    },
}
