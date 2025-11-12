use std::env;
use std::process::ExitCode;
use std::str::FromStr;
use rustoleum::{Unit, convert, TOLERANCE_EPSILON, TOLERANCE_ULPS};
use float_cmp::approx_eq;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("{} : USAGE <input units> <target units> <control> <answer>", &args[0]);
        return ExitCode::from(1);
    }

    // Parse input and target units using type-safe enums
    let Ok(uom_in) = Unit::from_str(&args[1]) else {
        println!("Answer: invalid");
        return ExitCode::from(1);
    };

    let Ok(uom_target) = Unit::from_str(&args[2]) else {
        println!("Answer: invalid");
        return ExitCode::from(1);
    };

    // Parse control and answer values
    let Ok(control) = args[3].parse::<f64>() else {
        println!("Answer: invalid");
        return ExitCode::from(1);
    };

    let Ok(answer) = args[4].parse::<f64>() else {
        println!("Answer: invalid");
        return ExitCode::from(1);
    };

    // Perform type-safe conversion
    let Some(expected) = convert(uom_in, uom_target, control) else {
        println!("Answer: invalid");
        return ExitCode::from(1);
    };

    if approx_eq!(f64, expected, answer, (TOLERANCE_EPSILON, TOLERANCE_ULPS)) {
        println!("Answer: correct");
        ExitCode::SUCCESS
    } else {
        println!("Answer: incorrect");
        ExitCode::SUCCESS
    }
}
