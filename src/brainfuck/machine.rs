use crate::brainfuck::BrainfuckError::{DataOverflow, DataPointerNegative, DataPointerOverflow};
use crate::brainfuck::{BrainfuckError, Commands};

const DATA_SIZE: usize = 30000;

pub(crate) struct Machine {
    commands: Vec<Commands>,
    data: [i8; 30000],
    instr_pointer: usize,
    data_pointer: usize,
}

impl Machine {
    pub(crate) fn create(commands: Vec<Commands>) -> Machine {
        Machine {
            commands,
            data: [0; DATA_SIZE],
            instr_pointer: 0,
            data_pointer: 0,
        }
    }

    pub(crate) fn execute(&mut self) -> Result<(), BrainfuckError> {
        while (self.instr_pointer as usize) < self.commands.len() {
            let last_command_result = match self.commands[self.instr_pointer] {
                Commands::IncDataPointer => self.inc_data_pointer(),
                Commands::DecDataPointer => self.dec_data_pointer(),
                Commands::IncData => self.inc_data(),
                Commands::DecData => self.dec_data(),
                Commands::Output => self.output(),
                Commands::Input => self.input(),
                Commands::StartBlock { next_instr } => self.jmp_z(next_instr.unwrap()),
                Commands::EndBlock { next_instr } => self.jmp_nz(next_instr.unwrap()),
            };

            if last_command_result.is_err() {
                return Err(last_command_result.err().unwrap());
            };

            self.instr_pointer += 1;
        }
        return Ok(());
    }

    fn inc_data_pointer(&mut self) -> Result<(), BrainfuckError> {
        self.data_pointer += 1;
        if self.data_pointer == DATA_SIZE {
            return Err(DataPointerOverflow);
        }
        return Ok(());
    }

    fn dec_data_pointer(&mut self) -> Result<(), BrainfuckError> {
        if self.data_pointer == 0 {
            return Err(DataPointerNegative);
        }
        self.data_pointer -= 1;
        return Ok(());
    }

    fn inc_data(&mut self) -> Result<(), BrainfuckError> {
        match self.data[self.data_pointer].checked_add(1) {
            None => Err(DataOverflow),
            Some(value) => {
                self.data[self.data_pointer] = value;
                Ok(())
            }
        }
    }

    fn dec_data(&mut self) -> Result<(), BrainfuckError> {
        match self.data[self.data_pointer].checked_sub(1) {
            None => Err(DataOverflow),
            Some(value) => {
                self.data[self.data_pointer] = value;
                Ok(())
            }
        }
    }

    fn output(&mut self) -> Result<(), BrainfuckError> {
        print!("{}", self.data[self.data_pointer]);
        return Ok(());
    }

    fn input(&mut self) -> Result<(), BrainfuckError> {
        todo!()
    }

    fn jmp_z(&mut self, next_instr: usize) -> Result<(), BrainfuckError> {
        if self.data[self.data_pointer] == 0 {
            self.instr_pointer = next_instr;
        }
        return Ok(());
    }

    fn jmp_nz(&mut self, next_instr: usize) -> Result<(), BrainfuckError> {
        if self.data[self.data_pointer] != 0 {
            self.instr_pointer = next_instr;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod machine_test;