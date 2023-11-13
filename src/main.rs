use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    process::exit,
};

mod assembler;
mod lmc;
mod logger;
mod numbers;

use assembler::Assembler;
use lmc::LMC;
use logger::{LogLevel, Logger};
use numbers::ThreeDigitNumber;

struct Test {
    name: String,
    input: Option<Vec<ThreeDigitNumber>>,
    result: Option<ThreeDigitNumber>,
    iterations: usize,
}

impl Test {
    pub fn new(
        name: &String,
        input: Option<Vec<ThreeDigitNumber>>,
        result: Option<ThreeDigitNumber>,
        iterations: usize,
    ) -> Test {
        Test {
            name: name.clone(),
            input,
            result,
            iterations,
        }
    }
}

fn main() {
    // Collect all arguments into a vector
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() < 2 {
        print_usage();
    }

    let logger = Logger::new(false, false);

    // Extract flags
    let flags = args
        .iter()
        .filter(|arg| arg.starts_with("-"))
        .map(|arg| arg.to_string())
        .map(|arg| arg.trim_start_matches("-").to_string())
        .collect::<Vec<String>>();
    // Check for help flag
    if flags.contains(&"h".to_string()) || flags.contains(&"help".to_string()) {
        print_usage();
    }
    // Check for other flags
    let verbose = flags.contains(&"v".to_string()) || flags.contains(&"verbose".to_string());
    let debug = flags.contains(&"d".to_string()) || flags.contains(&"debug".to_string());

    let commands = args
        .iter()
        .filter(|arg| !arg.starts_with("-"))
        .collect::<Vec<&String>>();
    if commands.len() < 2 {
        print_usage();
    }
    if commands.len() > 3 {
        print_usage();
    }

    // Execute the command
    let cmd = *commands.get(0).unwrap();
    if cmd == &"assemble".to_string() {
        let input_file = match commands.get(1) {
            Some(file) => file,
            None => {
                print_usage();
                return;
            }
        };
        let output_file = match commands.get(2) {
            Some(file) => file,
            None => {
                print_usage();
                return;
            }
        };
        let mut input = BufReader::new(File::open(input_file).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
        let asm = Assembler::new(verbose, debug);
        let program = match asm.assemble(&mut input) {
            Ok(program) => program,
            Err(err) => {
                logger.log(&LogLevel::Error, &format!("{}", err));
                exit(1);
            }
        };
        let mut output = File::create(output_file).unwrap();
        for instruction in program {
            writeln!(output, "{}", instruction.to_string()).unwrap();
        }
    } else if cmd == &"execute".to_string() {
        let program_file = match commands.get(1) {
            Some(file) => file,
            None => {
                print_usage();
                return;
            }
        };
        let input = parse_program_file(&logger, program_file);
        let mut lmc = LMC::new(verbose, debug, false);
        match lmc.load_program(&input) {
            Ok(_) => (),
            Err(err) => {
                logger.log(&LogLevel::Error, &format!("{}", err));
                exit(1);
            }
        }
        match lmc.execute_program() {
            Ok(_) => (),
            Err(err) => {
                logger.log(&LogLevel::Error, &format!("{}", err));
                exit(1);
            }
        }
    } else if cmd == &"batch".to_string() {
        let program_file = match commands.get(1) {
            Some(file) => file,
            None => {
                print_usage();
                return;
            }
        };
        let test_file = match commands.get(2) {
            Some(file) => file,
            None => {
                print_usage();
                return;
            }
        };
        let input = parse_program_file(&logger, program_file);
        let tests = parse_test_file(&logger, test_file);
        let mut lmc = LMC::new(verbose, debug, true);
        match lmc.load_program(&input) {
            Ok(_) => (),
            Err(err) => {
                logger.log(&LogLevel::Error, &format!("{}", err));
                exit(1);
            }
        }
        'outer: for test in tests {
            println!(
                "Running test: {} [{} iterations]",
                test.name, test.iterations
            );
            for _ in 0..test.iterations {
                match &test.input {
                    Some(input) => lmc.load_input(input),
                    None => (),
                }
                match lmc.execute_program() {
                    Ok(_) => (),
                    Err(err) => {
                        logger.log(&LogLevel::Error, &format!("{}", err));
                        exit(1);
                    }
                }
                let got = match lmc.get_output() {
                    Some(result) => format!("{:03}", result.value().to_string()),
                    None => "None".to_string(),
                };
                let expected = match test.result {
                    Some(result) => format!("{:03}", result.value().to_string()),
                    None => "None".to_string(),
                };
                if got != expected {
                    let inputs = match test.input {
                        Some(ref input) => input
                            .iter()
                            .map(|number| format!("{:03}", number.value().to_string()))
                            .collect::<Vec<String>>(),
                        None => vec![],
                    };
                    logger.log(
                        &LogLevel::Error,
                        &format!(
                            "[{}] Incorrect result for inputs [{:?}]: got {}, expected {}",
                            test.name, inputs, got, expected,
                        ),
                    );
                    break 'outer;
                }
                lmc.reset_counter();
            }
        }
    } else {
        print_usage();
    }
}

fn parse_program_file(logger: &Logger, program_file: &str) -> Vec<ThreeDigitNumber> {
    let input = BufReader::new(match File::open(program_file) {
        Ok(file) => file,
        Err(err) => {
            logger.log(&LogLevel::Error, &format!("{}", err));
            exit(1);
        }
    })
    .lines()
    .map(|line| match line {
        Ok(line) => line,
        Err(err) => {
            logger.log(&LogLevel::Error, &format!("{}", err));
            exit(1);
        }
    })
    .map(|line| match line.trim().parse::<i16>() {
        Ok(number) => number,
        Err(err) => {
            logger.log(&LogLevel::Error, &format!("{}", err));
            exit(1);
        }
    })
    .map(|instruction| match ThreeDigitNumber::new(instruction) {
        Ok(number) => number,
        Err(err) => {
            logger.log(&LogLevel::Error, &format!("{}", err));
            exit(1);
        }
    })
    .collect::<Vec<ThreeDigitNumber>>();
    input
}

fn parse_test_file(logger: &Logger, test_file: &str) -> Vec<Test> {
    let mut tests = Vec::new();
    let input = BufReader::new(match File::open(test_file) {
        Ok(file) => file,
        Err(err) => {
            logger.log(&LogLevel::Error, &format!("{}", err));
            exit(1);
        }
    })
    .lines()
    .map(|line| match line {
        Ok(line) => line,
        Err(err) => {
            logger.log(&LogLevel::Error, &format!("{}", err));
            exit(1);
        }
    });
    for line in input.into_iter() {
        let parts = line
            .trim()
            .split(";")
            .map(|part| part.to_string())
            .collect::<Vec<String>>();
        let parts_len = parts.len();
        if parts_len < 4 {
            logger.log(
                &LogLevel::Error,
                &format!("Invalid test file format: {}", line),
            );
            exit(1);
        }
        let name = &parts[0];
        let iterations = match parts[3].parse::<usize>() {
            Ok(iterations) => iterations,
            Err(err) => {
                logger.log(
                    &LogLevel::Error,
                    &format!("Invalid number of iterations: {}", err),
                );
                exit(1);
            }
        };
        let input_values = parts[1]
            .split(",")
            .map(|part| match part.parse::<i16>() {
                Ok(value) => value,
                Err(err) => {
                    logger.log(&LogLevel::Error, &format!("Invalid input value: {}", err));
                    exit(1);
                }
            })
            .map(|value| match ThreeDigitNumber::new(value) {
                Ok(number) => number,
                Err(err) => {
                    logger.log(&LogLevel::Error, &format!("{}", err));
                    exit(1);
                }
            })
            .collect::<Vec<ThreeDigitNumber>>();
        let test_result = match parts[2].parse::<i16>() {
            Ok(value) => match ThreeDigitNumber::new(value) {
                Ok(number) => Some(number),
                Err(err) => {
                    logger.log(&LogLevel::Error, &format!("{}", err));
                    exit(1);
                }
            },
            Err(_) => None,
        };
        if input_values.len() == 0 {
            tests.push(Test::new(&name, None, test_result, iterations));
        } else {
            tests.push(Test::new(
                &name,
                Some(input_values),
                test_result,
                iterations,
            ));
        }
    }

    tests
}

fn print_usage() {
    println!("Usage: lmc <command> <flags>");
    println!();
    println!("Commands:");
    println!("\tassemble <input file> <output file>");
    println!("\texecute <input file>");
    println!("\tbatch <program file> <batch file>");
    println!();
    println!("Flags:");
    println!("\t-h, --help\tShow this help message");
    println!("\t-v, --verbose\tShow verbose output");
    println!("\t-d, --debug\tShow debug output");
    exit(0);
}
