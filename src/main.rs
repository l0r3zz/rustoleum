use std::collections::HashMap;
use std::process::exit;
use std::env;
extern crate float_cmp;
use rustoleum::*;
use float_cmp::*;


// macro to make hasmap initialization easy
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {

    let args: Vec<String> = env::args().collect();
        if args.len() != 5 {
            println!("{} : USAGE <input units> <target units> <control> <answer>",&args[0]);
            exit(-1);
        }
    let  uom_in = &args[1];
    let  uom_target = &args[2];
    let  control = &args[3];
    let  answer = &args[4];
    // create a hashmap to contain the argument values
    let args_ctx = hashmap![
        "uom_in" => uom_in.to_ascii_uppercase(),
        "uom_target" => uom_target.to_ascii_uppercase(),
        "control" => control.to_string(),
        "answer" => answer.to_string()
    ];

    // These are the second level hashmaps that are values
    // for the keys of the first level hashmaps.
    // The first level hasmap keys are the "uom_in" value

    let kelvin_map = hashmap![
        "CELSIUS" => K2C,
        "FAHRENHEIT" => K2F,
        "RANKINE" => K2R
    ];

    let celsius_map = hashmap![
        "KELVIN" => C2K,
        "FAHRENHEIT" => C2K,
        "RANKINE" => C2R
    ];

    let fahrenheit_map = hashmap![
        "CELSIUS" => F2C,
        "KELVIN" => F2K,
        "RANKINE" => F2R
    ];

    let rankine_map = hashmap![
        "CELSIUS" => R2C,
        "FAHRENHEIT" => R2F,
        "KELVIN" => R2K
    ];

    let liters_map = hashmap![
        "TABLESPOONS" => L2T,
        "CUBIC-INCHES" => L2CI,
        "CUBIC-FEET" => L2CF,
        "CUPS" => L2CPS,
        "GALLONS" => L2G
    ];

    let tablespoons_map = hashmap![
        "LITERS" => T2L,
        "CUBIC-INCHES" => T2CI,
        "CUBIC-FEET" => T2CF,
        "CUPS" => T2CPS,
        "GALLONS" => T2G
    ];

    let cubic_feet_map = hashmap![
        "LITERS" => CF2L,
        "CUBIC-INCHES" => CF2CI,
        "TABLESPOONS" => CF2T,
        "CUPS" => CF2CPS,
        "GALLONS" => CF2G
    ];

    let cubic_inches_map = hashmap![
        "LITERS" => CI2L,
        "CUBIC-FEET" => CI2CF,
        "TABLESPOONS" => CI2T,
        "CUPS" => CI2CPS,
        "GALLONS" => CI2G
    ];

    let cups_map = hashmap![
        "LITERS" => CPS2L,
        "CUBIC-INCHES" => CPS2CI,
        "TABLESPOONS" => CPS2T,
        "CUBIC-FEET" => CPS2CF,
        "GALLONS" => CPS2G
    ];

    let gallons_map = hashmap![
        "LITERS" => G2L,
        "CUBIC-INCHES" => G2CI,
        "TABLESPOONS" => G2T,
        "CUBIC-FEET" => G2CF,
        "CUPS" => G2CPS
    ];

    // Main conversion dispatch table
    // This is the top level conversion table, if there is a hash "hit"
    // that gurentees that the input Unit of Measure (uom_in) is valid
    let cvnmap = hashmap![
        "KELVIN" => kelvin_map,
        "CELSIUS" => celsius_map,
        "FAHRENHEIT" => fahrenheit_map,
        "RANKINE" => rankine_map,
        "LITERS" => liters_map,
        "TABLESPOONS" => tablespoons_map,
        "CUBIC-INCHES" => cubic_inches_map,
        "CUPS" => cups_map,
        "CUBIC-FEET" => cubic_feet_map,
        "GALLONS" => gallons_map
    ];

    // All of the work is done here,
    //  Probe the cvnmap hashmap for the provided "uom_in" value, if not found
    //  return "invalid", if found hash into the value returned (which is a hashmap)
    //  with the provided "uom_target" as a key. If the key is found, apply the "control"
    //  argument to the dereferenced function pointer and compare the delivered f64 value
    //  with the provided "answer" if they are approximately equal return correct otherwise
    //  return incorrect. If either "control" or "answer" do not parse to f64 types, return
    //  invalid

    match cvnmap.get(&*args_ctx["uom_in"]){
        Some(value) => {
            match value.get(&*args_ctx["uom_target"]){
                Some(r) => {
                    let cntrf64 = match  args_ctx["control"].parse::<f64>() {
                        Ok(result) => result,
                        Err(_err) => {println!("Answer: invalid");return() }
                    };
                    let ansf64 = match  args_ctx["answer"].parse::<f64>() {
                        Ok(result) => result,
                        Err(_err) => {println!("Answer: invalid");return() }
                    };
                    if approx_eq!(f64,r(cntrf64), ansf64,(0.005,2)){
                    println!("Answer: {}","correct");
                    }else {
                    println!("Answer: {}","incorrect");
                    }
                }
                None => {
                    println!("Answer: {}","invalid");
                }
            }
        }
        None => {
            println!("Answer: {}","invalid");
        }
    }
}
