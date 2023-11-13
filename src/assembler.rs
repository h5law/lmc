use regex::Regex;
use std::{collections::HashMap, fmt};

use crate::{
    logger::{LogLevel, Logger},
    numbers::ThreeDigitNumber,
};

// AssemblerError is used to indicate an error with the assembler
pub enum AssemblerError {
    InvalidOpcode(String),
    InvalidLabel(String),
    InvalidNumberOfMneumonics(usize, String),
    EmptyInput,
    TooManyLinesOfInput(usize),
}

// Implement the display trait for easy printing.
impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerError::InvalidOpcode(opcode) => {
                write!(f, "invalid opcode: got {}", opcode)
            }
            AssemblerError::InvalidLabel(label) => write!(f, "invalid label: got {}", label),
            AssemblerError::InvalidNumberOfMneumonics(index, line) => {
                write!(
                    f,
                    "invalid number of mneumonics in line {}: {}",
                    index, line
                )
            }
            AssemblerError::EmptyInput => write!(f, "empty input"),
            AssemblerError::TooManyLinesOfInput(lines) => {
                write!(f, "too many lines of input: got {}", lines)
            }
        }
    }
}

// OPCODES are the opcodes for the LMC
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

impl OPCODES {
    // to_number converts an opcode to a ThreeDigitNumber
    pub fn to_number(&self) -> ThreeDigitNumber {
        match self {
            OPCODES::ADD => ThreeDigitNumber::new(100).unwrap(),
            OPCODES::SUB => ThreeDigitNumber::new(200).unwrap(),
            OPCODES::STO => ThreeDigitNumber::new(300).unwrap(),
            OPCODES::LDA => ThreeDigitNumber::new(500).unwrap(),
            OPCODES::BR => ThreeDigitNumber::new(600).unwrap(),
            OPCODES::BRZ => ThreeDigitNumber::new(700).unwrap(),
            OPCODES::BRP => ThreeDigitNumber::new(800).unwrap(),
            OPCODES::IN => ThreeDigitNumber::new(901).unwrap(),
            OPCODES::OUT => ThreeDigitNumber::new(902).unwrap(),
            OPCODES::HLT => ThreeDigitNumber::new(000).unwrap(),
            OPCODES::DAT => ThreeDigitNumber::new(000).unwrap(),
        }
    }

    // from_str converts a string to an opcode
    pub fn from_str(opcode: &str) -> Result<OPCODES, AssemblerError> {
        match opcode {
            "ADD" => Ok(OPCODES::ADD),
            "SUB" => Ok(OPCODES::SUB),
            "STO" => Ok(OPCODES::STO),
            "LDA" => Ok(OPCODES::LDA),
            "BR" => Ok(OPCODES::BR),
            "BRZ" => Ok(OPCODES::BRZ),
            "BRP" => Ok(OPCODES::BRP),
            "IN" => Ok(OPCODES::IN),
            "OUT" => Ok(OPCODES::OUT),
            "HLT" => Ok(OPCODES::HLT),
            "DAT" => Ok(OPCODES::DAT),
            _ => Err(AssemblerError::InvalidOpcode(opcode.to_string())),
        }
    }
}

// Assembler is used to assemble LMC programs
pub struct Assembler {
    // logger is used to log messages to the console
    logger: Logger,
}

impl Assembler {
    // new creates a new Assembler instance
    pub fn new(verbose: bool, debug: bool) -> Self {
        Assembler {
            logger: Logger::new(verbose, debug),
        }
    }

    // assemble assembles a program in the form of a vector of strings
    // into a vector of ThreeDigitNumbers representing the LMC's mailboxes
    pub fn assemble(
        self: &Self,
        input: &mut Vec<String>,
    ) -> Result<Vec<ThreeDigitNumber>, AssemblerError> {
        self.logger
            .log(&LogLevel::Info, "assembling program into machine code...");
        // Create a hashmap for labels
        let mut labels: HashMap<String, usize> = HashMap::new();
        // Compile a regex to strip comments
        let comment_regex = Regex::new(r"#.*$").unwrap();
        // Strip comments and trim whitespace left over
        self.logger.log(&LogLevel::Debug, "stripping comments...");
        let mut stripped_input = input
            .into_iter()
            .map(|line| comment_regex.replace_all(line, "").trim().to_string())
            .collect::<Vec<String>>();
        // Remove empty lines
        self.logger.log(&LogLevel::Debug, "removing empty lines...");
        stripped_input.retain(|line| line.len() > 0);
        if stripped_input.len() == 0 {
            return Err(AssemblerError::EmptyInput);
        }
        // Check for too many lines of input
        if stripped_input.len() > 100 {
            return Err(AssemblerError::TooManyLinesOfInput(stripped_input.len()));
        }
        // TODO: Simplify and optimise this 2-pass strategy
        self.logger.log(&LogLevel::Info, "starting first pass...");
        for i in 0..stripped_input.len() {
            let line = &stripped_input[i];
            // Split the line into its parts. A line of LMC assembly can have up to 3 distinct
            // parts: a label, an opcode, and an operand. The label is optional, but the opcode
            // is not. Depending on the opcode, the operand may be optional.
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            match parts.len() {
                1 => {} // Just an opcode - no labels or operands
                // Two parts - either a label and an opcode, or an opcode and an operand
                2 => {
                    // Check if the first part is an opcode
                    match OPCODES::from_str(parts[0]) {
                        Ok(_) => {} // No label to collect
                        Err(_) => {
                            // The first part is not an opcode, so it must be a label
                            // add the label to the hashmap with its index for later use
                            match OPCODES::from_str(parts[1]) {
                                Ok(_) => {
                                    labels.insert(parts[0].to_string(), i);
                                    self.logger.log(
                                        &LogLevel::Debug,
                                        format!("inserting label {} at index {}", parts[0], i)
                                            .as_str(),
                                    );
                                }
                                Err(e) => return Err(e),
                            };
                        }
                    };
                }
                // Three parts - a label, an opcode, and an operand
                3 => {
                    // Insert the label into the hashmap with its index for later use
                    labels.insert(parts[0].to_string(), i);
                    self.logger.log(
                        &LogLevel::Debug,
                        format!("inserting label {} at index {}", parts[0], i).as_str(),
                    );
                }
                // Anything else is invalid
                n => {
                    return Err(AssemblerError::InvalidNumberOfMneumonics(
                        n,
                        line.to_string(),
                    ));
                }
            }
        }
        self.logger.log(&LogLevel::Info, "starting second pass...");
        let mut result = vec![ThreeDigitNumber::new(0).unwrap(); stripped_input.len()];
        for (i, line) in stripped_input.iter().enumerate() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let opcode: OPCODES;
            match parts.len() {
                // One part - just an opcode
                1 => {
                    // Convert the opcode to a ThreeDigitNumber and add it to the result
                    // vector at the current index
                    opcode = OPCODES::from_str(parts[0])?;
                    result[i] = opcode.to_number();
                    self.logger.log(
                        &LogLevel::Debug,
                        format!("{}:\t{}", i, opcode.to_number()).as_str(),
                    );
                }
                // Two parts - either a label and an opcode, or an opcode and an operand
                2 => {
                    match OPCODES::from_str(parts[0]) {
                        // First part is an opcode
                        Ok(_) => {
                            // Retrieve the opcode and and the index the operand refers to
                            // from the hashmap of labels
                            opcode = OPCODES::from_str(parts[0])?;
                            let label = match labels.get(parts[1]) {
                                Some(i) => i,
                                None => {
                                    return Err(AssemblerError::InvalidLabel(parts[1].to_string()));
                                }
                            };
                            // Convert the index to a ThreeDigitNumber and add it to the
                            // opcode to get the final instruction's ThreeDigitNumber value
                            let value = ThreeDigitNumber::new(*label as i16).unwrap();
                            let instruction = opcode.to_number() + value;
                            result[i] = instruction.unwrap();
                            self.logger.log(
                                &LogLevel::Debug,
                                format!("{}:\t{}", i, (opcode.to_number() + value).unwrap())
                                    .as_str(),
                            );
                        }
                        // First part is a label
                        Err(_) => {
                            // Retrieve the opcode and convert it to a ThreeDigitNumber
                            match OPCODES::from_str(parts[1]) {
                                Ok(op) => {
                                    result[i] = op.to_number();
                                }
                                Err(e) => return Err(e),
                            };
                        }
                    };
                }
                // Three parts - a label, an opcode, and an operand
                3 => {
                    // Retrieve the opcode
                    opcode = OPCODES::from_str(parts[1])?;
                    match opcode {
                        // DAT is a special case and is used to signify a data storage location
                        // rather than an instruction. The operand is the value to store in the
                        // mailbox at the current index.
                        OPCODES::DAT => {
                            let value =
                                ThreeDigitNumber::new(parts[2].parse::<i16>().unwrap()).unwrap();
                            result[i] = value;
                            self.logger
                                .log(&LogLevel::Debug, format!("{}:\t{}", i, value).as_str());
                            continue;
                        }
                        // Otherwise, the operand is an index to a label in the hashmap so continue
                        _ => {}
                    };
                    // Retrieve the index the operand refers to from the hashmap of labels
                    let label = match labels.get(parts[2]) {
                        Some(i) => i,
                        None => {
                            return Err(AssemblerError::InvalidLabel(parts[2].to_string()));
                        }
                    };
                    // Convert the index to a ThreeDigitNumber and add it to the opcode to get
                    // the final instruction's ThreeDigitNumber value and add it to the result
                    let value = ThreeDigitNumber::new(*label as i16).unwrap();
                    let instruction = opcode.to_number() + value;
                    result[i] = instruction.unwrap();
                    self.logger.log(
                        &LogLevel::Debug,
                        format!("{}:\t{}", i, (opcode.to_number() + value).unwrap()).as_str(),
                    );
                }
                _ => {}
            }
        }

        Ok(result)
    }
}
