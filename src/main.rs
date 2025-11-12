use std::env;
use std::process::ExitCode;
use std::str::FromStr;
use rustoleum::{Unit, convert, TOLERANCE_EPSILON, TOLERANCE_ULPS};
use float_cmp::*;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("{} : USAGE <input units> <target units> <control> <answer>", &args[0]);
        return ExitCode::from(1);
    }

    // Parse input and target units using type-safe enums
    let uom_in = match Unit::from_str(&args[1]) {
        Ok(unit) => unit,
        Err(_) => {
            println!("Answer: invalid");
            return ExitCode::from(1);
        }
    };

    let uom_target = match Unit::from_str(&args[2]) {
        Ok(unit) => unit,
        Err(_) => {
            println!("Answer: invalid");
            return ExitCode::from(1);
        }
    };

    // Parse control and answer values
    let control = match args[3].parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Answer: invalid");
            return ExitCode::from(1);
        }
    };

    let answer = match args[4].parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Answer: invalid");
            return ExitCode::from(1);
        }
    };

    // Perform type-safe conversion
    match convert(uom_in, uom_target, control) {
        Some(expected) => {
            if approx_eq!(f64, expected, answer, (TOLERANCE_EPSILON, TOLERANCE_ULPS)) {
                println!("Answer: correct");
                ExitCode::SUCCESS
            } else {
                println!("Answer: incorrect");
                ExitCode::SUCCESS
            }
        }
        None => {
            println!("Answer: invalid");
            ExitCode::from(1)
        }
    }
}
