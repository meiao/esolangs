use crate::brainfuck::Error::{MismatchedClose, MismatchedOpen};

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

#[derive(Debug)]
pub enum Error {
    MismatchedOpen,
    MismatchedClose,
}

pub fn _execute(source: String) {
    let commands = parse(source);
    match commands {
        Ok(commands) => run(commands),
        Err(error) => exit(error),
    }
}

pub fn run(commands: Vec<Commands>) {
    Machine::create(commands).execute()
}

fn parse(source: String) -> Result<Vec<Commands>, Error> {
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
                commands.push(Commands::StartBlock { next_instr: None });
                opens.push(commands.len() - 1)
            }
            ']' => {
                if opens.is_empty() {
                    return Err(MismatchedClose);
                }
                let open = opens.pop().unwrap();
                commands.push(Commands::EndBlock {
                    next_instr: Some(open),
                });
                commands[open] = Commands::StartBlock {
                    next_instr: Some(commands.len() - 1),
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

struct Machine {
    commands: Vec<Commands>,
    data: [i8; 30000],
    instr_pointer: usize,
    data_pointer: usize,
}

impl Machine {
    fn create(commands: Vec<Commands>) -> Machine {
        Machine {
            commands,
            data: [0; 30000],
            instr_pointer: 0,
            data_pointer: 0,
        }
    }

    fn execute(&mut self) {
        while (self.instr_pointer as usize) < self.commands.len() {
            match self.commands[self.instr_pointer] {
                Commands::IncDataPointer => self.inc_data_pointer(),
                Commands::DecDataPointer => self.dec_data_pointer(),
                Commands::IncData => self.inc_data(),
                Commands::DecData => self.dec_data(),
                Commands::Output => self.output(),
                Commands::Input => self.input(),
                Commands::StartBlock { next_instr } => self.jmp_z(next_instr.unwrap()),
                Commands::EndBlock { next_instr } => self.jmp_nz(next_instr.unwrap()),
            }
            self.instr_pointer += 1;
        }
    }
    fn inc_data_pointer(&mut self) {
        self.data_pointer += 1
    }

    fn dec_data_pointer(&mut self) {
        self.data_pointer -= 1
    }

    fn inc_data(&mut self) {
        self.data[self.data_pointer] += 1
        // todo check if pointer is valid and if there is overflow
    }

    fn dec_data(&mut self) {
        self.data[self.data_pointer] -= 1
        // todo check if pointer is valid and if there is overflow
    }

    fn output(&mut self) {
        print!("{}", self.data[self.data_pointer])
    }

    fn input(&mut self) {
        todo!()
    }

    fn jmp_z(&mut self, next_instr: usize) {
        if self.data[self.data_pointer] == 0 {
            self.instr_pointer = next_instr;
        }
    }

    fn jmp_nz(&mut self, next_instr: usize) {
        if self.data[self.data_pointer] != 0 {
            self.instr_pointer = next_instr;
        }
    }
}

fn exit(_err: Error) {
    print!("An error occurred.");
    std::process::exit(1);
}

#[cfg(test)]
mod test;
