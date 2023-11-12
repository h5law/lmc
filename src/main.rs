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
        let input_file = match commands.get(1) {
            Some(file) => file,
            None => {
                print_usage();
                return;
            }
        };
        let mut input = BufReader::new(match File::open(input_file) {
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
        let mut lmc = LMC::new(verbose, debug);
        match lmc.load_program(&mut input) {
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
    } else {
        print_usage();
    }
}

fn print_usage() {
    println!("Usage: lmc <command> <flags>");
    println!();
    println!("Commands:");
    println!("\tassemble <input file> <output file>");
    println!("\texecute <input file>");
    println!();
    println!("Flags:");
    println!("\t-h, --help\tShow this help message");
    println!("\t-v, --verbose\tShow verbose output");
    println!("\t-d, --debug\tShow debug output");
    exit(0);
}
