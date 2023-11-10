use std::{
    collections::VecDeque,
    fs::File,
    io::{prelude::*, stdin, stdout, BufReader},
};

mod errors;
mod numbers;

use errors::LMCError;
use numbers::{Flag, ThreeDigitNumber, TwoDigitNumber};

struct LMC {
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
}

enum OPCODES {
    ADD, // 1xx ADDITION
    SUB, // 2xx SUBTRACT
    STO, // 3xx STORE
    LDA, // 5xx LOAD
    BR,  // 6xx BRANCH
    BRZ, // 7xx BRANCH ZERO
    BRP, // 8xx BRANCH POSITIVE
    IN,  // 901 INPUT
    OUT, // 902 OUTPUT
    HLT, // 000 HALT
    DAT, //     DATA STORAGE LOCATION
}

impl LMC {
    pub fn new() -> Self {
        LMC {
            mailboxes: [ThreeDigitNumber::new(0).unwrap(); 100],
            calculator: ThreeDigitNumber::new(0).unwrap(),
            in_basket: VecDeque::new(),
            out_basket: None,
            counter: TwoDigitNumber::new(0).unwrap(),
            flag: None,
        }
    }

    pub fn load_program_from_file(self: &mut Self, file: String) -> Result<(), LMCError> {
        let file = match File::open(file) {
            Ok(file) => file,
            Err(e) => return Err(LMCError::IOError(e.to_string())),
        };
        let reader = BufReader::new(file);
        let mut program: Vec<ThreeDigitNumber> = Vec::new();
        for line in reader.lines() {
            let line_string = match line {
                Ok(line) => line,
                Err(e) => return Err(LMCError::IOError(e.to_string())),
            };
            let trimmed = line_string.trim();
            let parts = trimmed.split(",");
            for part in parts {
                match part.parse::<i16>() {
                    Ok(number) => match ThreeDigitNumber::new(number) {
                        Ok(number) => program.push(number),
                        Err(e) => return Err(LMCError::NumberError(e.to_string())),
                    },
                    Err(e) => return Err(LMCError::IOError(e.to_string())),
                }
            }
        }
        return self.load_program(&program);
    }

    pub fn load_program(self: &mut Self, program: &Vec<ThreeDigitNumber>) -> Result<(), LMCError> {
        if program.len() > 100 {
            return Err(LMCError::ProgramTooLarge(format!(
                "{} > 100",
                program.len()
            )));
        }
        for (i, instruction) in program.iter().enumerate() {
            self.mailboxes[i] = *instruction;
        }
        Ok(())
    }

    pub fn execute_program(self: &mut Self) -> Result<(), LMCError> {
        loop {
            let instruction = self.mailboxes[self.counter.value() as usize];
            let opcode = instruction.value() / 100;
            let operand = (instruction.value() % 100) as usize;
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
                    _ => return Err(LMCError::InvalidOpcode(format!("9{:02}", opcode))),
                },
                0 => return Ok(()),
                _ => return Err(LMCError::InvalidOpcode(format!("{:03}", opcode))),
            }
        }
    }

    fn add(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.mailboxes[operand];
        self.calculator += value;
        match self.calculator.flag() {
            Some(flag) => self.flag = Some(flag),
            None => self.flag = None,
        }
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(LMCError::NumberError(e)),
        };
        Ok(())
    }

    fn sub(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.mailboxes[operand];
        self.calculator -= value;
        match self.calculator.flag() {
            Some(flag) => self.flag = Some(flag),
            None => self.flag = None,
        }
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(LMCError::NumberError(e)),
        };
        Ok(())
    }

    fn sto(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.calculator;
        self.mailboxes[operand] = value;
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(LMCError::NumberError(e)),
        };
        Ok(())
    }

    fn lda(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        let value = self.mailboxes[operand];
        self.calculator = value;
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(LMCError::NumberError(e)),
        };
        Ok(())
    }

    fn br(self: &mut Self, operand: usize) {
        self.counter = TwoDigitNumber::new(operand as u8).unwrap();
    }

    fn brz(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        if self.calculator.value() == 0 {
            self.counter = TwoDigitNumber::new(operand as u8).unwrap();
        } else {
            self.counter += match TwoDigitNumber::new(1) {
                Ok(number) => number,
                Err(e) => return Err(LMCError::NumberError(e)),
            };
        }
        Ok(())
    }

    fn brp(self: &mut Self, operand: usize) -> Result<(), LMCError> {
        match self.flag {
            Some(flag) => match flag {
                Flag::NEG => {
                    self.counter += match TwoDigitNumber::new(1) {
                        Ok(number) => number,
                        Err(e) => return Err(LMCError::NumberError(e)),
                    };
                    return Ok(());
                }
                _ => {
                    self.counter = match TwoDigitNumber::new(operand as u8) {
                        Ok(number) => number,
                        Err(e) => return Err(LMCError::NumberError(e)),
                    };
                    return Ok(());
                }
            },
            None => {
                self.counter = match TwoDigitNumber::new(operand as u8) {
                    Ok(number) => number,
                    Err(e) => return Err(LMCError::NumberError(e)),
                };
                return Ok(());
            }
        }
    }

    fn read_input(self: &mut Self) -> Result<(), LMCError> {
        self.calculator = match self.in_basket.pop_front() {
            Some(number) => number,
            None => match self.read_blocking() {
                Ok(number) => number,
                Err(e) => return Err(e),
            },
        };
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(LMCError::NumberError(e)),
        };
        Ok(())
    }

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
                Err(e) => return Err(LMCError::NumberError(e.to_string())),
            },
            Err(e) => return Err(LMCError::IOError(e.to_string())),
        }
    }

    fn write_output(self: &mut Self) -> Result<(), LMCError> {
        self.out_basket = Some(self.calculator);
        self.counter += match TwoDigitNumber::new(1) {
            Ok(number) => number,
            Err(e) => return Err(LMCError::NumberError(e)),
        };
        Ok(())
    }

    pub fn show_output(self: &Self) {
        match self.out_basket {
            Some(number) => println!("Output: {}", number.value()),
            None => println!("No output"),
        }
    }
}

fn main() {
    let mut lmc = LMC::new();
    println!("Loading program...");
    match lmc.load_program_from_file("program.txt".to_string()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    println!("Executing program...");
    match lmc.execute_program() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
}
