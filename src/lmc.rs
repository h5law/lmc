use std::{
    collections::VecDeque,
    fmt,
    io::{stdin, stdout, Write},
};

use crate::{
    logger::{LogLevel, Logger},
    numbers::{Flag, NumberError, ThreeDigitNumber, TwoDigitNumber},
};

// LMCError is used to indicate an error with the LMC VM
#[derive(Debug, PartialEq)]
pub enum LMCError {
    NumberError(NumberError),
    ProgramTooLarge(usize),
    IOError(String),
    InvalidOpcode(String),
}

// Implement the display trait for easy printing.
impl fmt::Display for LMCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LMCError::ProgramTooLarge(value) => {
                write!(f, "program too large: got {} instructions", value)
            }
            LMCError::IOError(value) => write!(f, "IO error: {}", value.to_string()),
            LMCError::InvalidOpcode(value) => write!(f, "invalid opcode: {}", value),
            LMCError::NumberError(value) => write!(f, "number error: {}", value.to_string()),
        }
    }
}

// Implement the from trait for NumberError.
impl From<NumberError> for LMCError {
    fn from(error: NumberError) -> Self {
        LMCError::NumberError(error)
    }
}

// LMC defines the structure of the Little Minion Computer and is the VM
// responsible for executing any programs. The LMC is a toy-example of a
// computer architecture used to teach the fundamentals of architectures
// and assembly language in general. It is not a real computer.
pub struct LMC {
    // mailboxes hold 3-digit decimal numbers in each 100 address
    mailboxes: [ThreeDigitNumber; 100],
    // calculator holds a 3-digit decimal number used in calculations
    // and as an intermediate memory location for certain op-codes
    calculator: ThreeDigitNumber,
    // in_basket is a queue of 3-digit decimal numbers
    in_basket: VecDeque<ThreeDigitNumber>,
    // out_basket is an optional 3-digit decimal number
    out_basket: Option<ThreeDigitNumber>,
    // 2-digit counter is the program counter and provides the indexes
    // for the mailboxes during the fetch-execute cycle
    counter: TwoDigitNumber,
    // flag holds the current flag if any raised by the last operation
    flag: Option<Flag>,
    // logger is used to log messages to the console
    logger: Logger,
}

impl LMC {
    // new creates a new LMC with all values initialized to 0
    pub fn new(verbose: bool, debug: bool) -> Self {
        LMC {
            mailboxes: [ThreeDigitNumber::new(0).unwrap(); 100],
            calculator: ThreeDigitNumber::new(0).unwrap(),
            in_basket: VecDeque::new(),
            out_basket: None,
            counter: TwoDigitNumber::new(0).unwrap(),
            flag: None,
            logger: Logger::new(verbose, debug),
        }
    }

    // load_program loads an assembled program into the LMC's mailboxes ready for execution
    // NOTE: This does not verify the program is valid only that it is not too large
    pub fn load_program(self: &mut Self, program: &Vec<ThreeDigitNumber>) -> Result<(), LMCError> {
        self.logger.log(
            &LogLevel::Info,
            &format!("Loading program with {} instructions", program.len()),
        );
        if program.len() > 100 {
            return Err(LMCError::ProgramTooLarge(program.len()));
        }
        for (i, instruction) in program.iter().enumerate() {
            self.mailboxes[i] = *instruction;
        }
        Ok(())
    }

    // execute_program executes the program loaded into the LMC's mailboxes, iterating
    // through each instruction and executing it. The program counter is incremented
    // after each instruction is executed and the program exits when the counter
    // reaches the end of the program, signified by a 000 instruction.
    pub fn execute_program(self: &mut Self) -> Result<(), LMCError> {
        self.logger.log(&LogLevel::Info, "executing program...");
        // loop infinitely until we reach the end of the program
        loop {
            // fetch the instruction from the mailbox at the counter
            let instruction = self.mailboxes[self.counter.value() as usize];
            // retrieve the opcode and operand from the instruction
            let opcode = instruction.value() / 100;
            let operand = (instruction.value() % 100) as usize;
            // execute the instruction
            self.logger.log(
                &LogLevel::Debug,
                &format!(
                    "executing instruction: {:03} (opcode: {:01}, operand: {:02})",
                    instruction, opcode, operand
                ),
            );
            match opcode {
                1 => match self.add(operand) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                2 => match self.sub(operand) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                3 => match self.sto(operand) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                5 => match self.lda(operand) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                6 => self.br(operand),
                7 => match self.brz(operand) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                8 => match self.brp(operand) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                9 => match operand {
                    1 => match self.read_input() {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    },
                    2 => {
                        match self.write_output() {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                        self.show_output();
                    }
                    // there are only 2 I/O opcodes so any other is invalid
                    _ => return Err(LMCError::InvalidOpcode(format!("9{:02}", opcode))),
                },
                // 0 is the halt instruction and signifies the end of the program
                0 => return Ok(()),
                // any other opcode is invalid
                _ => return Err(LMCError::InvalidOpcode(format!("{:03}", opcode))),
            }
        }
    }

    // add adds the value in the mailbox at the operand to the calculator
    fn add(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.mailboxes[operand];
        self.logger.log(
            &LogLevel::Debug,
            &format!(
                "adding: {} + {}",
                self.calculator.to_string(),
                value.to_string()
            ),
        );
        self.calculator += value;
        match self.calculator.flag() {
            Some(flag) => {
                self.logger.log(
                    &LogLevel::Debug,
                    &format!("setting flag: {}", flag.to_string()),
                );
                self.flag = Some(flag)
            }
            None => self.flag = None,
        }
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    // sub subtracts the value at the operand from the calculator
    fn sub(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.mailboxes[operand];
        self.logger.log(
            &LogLevel::Debug,
            &format!(
                "subtracting: {} - {}",
                self.calculator.to_string(),
                value.to_string()
            ),
        );
        self.calculator -= value;
        match self.calculator.flag() {
            Some(flag) => {
                self.logger.log(
                    &LogLevel::Debug,
                    &format!("setting flag: {}", flag.to_string()),
                );
                self.flag = Some(flag)
            }
            None => self.flag = None,
        }
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    // sto stores the value in the calculator into the mailbox at the operand
    fn sto(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.calculator;
        self.mailboxes[operand] = value;
        self.logger.log(
            &LogLevel::Debug,
            &format!("storing to {}: {}", operand as u8, value.to_string()),
        );
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    // lda loads the value from the mailbox at the operand into the calculator
    fn lda(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.mailboxes[operand];
        self.calculator = value;
        self.logger.log(
            &LogLevel::Debug,
            &format!("loading from {}: {}", operand as u8, value.to_string()),
        );
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    // br sets the program counter to the operand (branch unconditional)
    fn br(self: &mut Self, operand: usize) {
        self.logger.log(
            &LogLevel::Debug,
            &format!("setting counter to {}", operand as u8),
        );
        self.counter = TwoDigitNumber::new(operand as u8).unwrap();
    }

    // brz sets the program counter to the operand if the calculator is 0
    // if the calculator is not 0 then the counter is incremented by 1 (branch zero)
    fn brz(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        if self.calculator.value() == 0 {
            self.logger.log(
                &LogLevel::Debug,
                &format!("setting counter to {}", operand as u8),
            );
            self.counter = TwoDigitNumber::new(operand as u8).unwrap();
        } else {
            self.counter += match TwoDigitNumber::new(1) {
                Ok(number) => number,
                Err(e) => return Err(e.into()),
            };
        }
        Ok(())
    }

    // brp sets the program counter to the operand if the LMC's flag is not NEG
    // if the flag is NEG then the counter is incremented by 1 (branch positive)
    fn brp(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        match self.flag {
            Some(flag) => match flag {
                Flag::NEG => {
                    self.counter += match TwoDigitNumber::new(1) {
                        Ok(number) => number,
                        Err(e) => return Err(e.into()),
                    };
                    return Ok(());
                }
                _ => {
                    self.counter = match TwoDigitNumber::new(operand as u8) {
                        Ok(number) => {
                            self.logger
                                .log(&LogLevel::Debug, &format!("setting counter to {}", number));
                            number
                        }
                        Err(e) => return Err(e.into()),
                    };
                    return Ok(());
                }
            },
            None => {
                self.counter = match TwoDigitNumber::new(operand as u8) {
                    Ok(number) => {
                        self.logger
                            .log(&LogLevel::Debug, &format!("setting counter to {}", number));
                        number
                    }
                    Err(e) => return Err(e.into()),
                };
                return Ok(());
            }
        }
    }

    // read_input reads a 3-digit decimal number from the input_tray or if
    // the tray is empty then it reads from stdin blocking until input is
    // received. It will error on invalid input.
    fn read_input(self: &mut Self) -> Result<(), LMCError> {
        self.calculator = match self.in_basket.pop_front() {
            Some(number) => number,
            None => match self.read_blocking() {
                Ok(number) => number,
                Err(e) => return Err(e.into()),
            },
        };
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    // read_blocking reads a 3-digit decimal number from stdin blocking
    // until input is received. It will error on invalid input.
    fn read_blocking(self: &Self) -> Result<ThreeDigitNumber, LMCError> {
        print!("Input: ");
        match stdout().flush() {
            Ok(_) => {}
            Err(e) => return Err(LMCError::IOError(e.to_string())),
        }
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => return Err(LMCError::IOError(e.to_string())),
        }
        let trimmed = input.trim();
        match trimmed.parse::<i16>() {
            Ok(number) => match ThreeDigitNumber::new(number) {
                Ok(number) => return Ok(number),
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(LMCError::IOError(e.to_string())),
        }
    }

    // write_output writes the value in the calculator to the output_tray
    fn write_output(self: &mut Self) -> Result<(), LMCError> {
        self.out_basket = Some(self.calculator);
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    // show_output prints the value in the output_tray to stdout
    pub fn show_output(self: &Self) {
        match self.out_basket {
            Some(number) => println!("Output: {}", number.value()),
            None => println!("No output"),
        }
    }
}
