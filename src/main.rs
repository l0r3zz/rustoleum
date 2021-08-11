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

 // function pointer type
type Measureop = fn(f64) -> f64;

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


    // Created function pointers which will go into the second level hashmaps

    // kelvin to celsius conversion function
    let k2c: Measureop = kel_cel;
    // kelvin to fahrenheit conversion function
    let k2f: Measureop = kel_fah;
    //kelvin to rankine conversion function
    let k2r: Measureop = kel_ran;
    //celsius to kelvin conversion function
    let c2k: Measureop = cel_kel;
    //celsius to fahrenheit conversion function
    let c2f: Measureop = cel_fah;
    //celsius to rankine conversion function
    let c2r: Measureop = cel_ran;
    //fahrenheit to rankine conversion function
    let f2r: Measureop = fah_ran;
    //fahrenheit to celsius conversion function
    let f2c: Measureop = fah_cel;
    //fahrenheit to kelvin conversion function
    let f2k: Measureop = fah_kel;
    //rankin to fahrenheit conversion function
    let r2f: Measureop = ran_fah;
    //rankin to celsius conversion function
    let r2c: Measureop = ran_cel;
    //rankin to kelvin conversion function
    let r2k: Measureop = ran_kel;
    //liter to tablespoon conversion function
    let l2t: Measureop = lit_tab;
    //liter to cubic-inches conversion function
    let l2ci: Measureop = lit_ci;
    //liter to cups conversion function
    let l2cps: Measureop = lit_cups;
    //liter to cubic-feet conversion function
    let l2cf: Measureop = lit_cf;
    //liter to gallons conversion function
    let l2g: Measureop = lit_gal;
    //tablespoons to liters conversion function
    let t2l: Measureop = tab_lit;
    //tablespoons to cubic-inches conversion function
    let t2ci: Measureop = tab_ci;
    //tablespoons to cups conversion function
    let t2cps: Measureop = tab_cups;
    //tablespoons to cubic-feet conversion function
    let t2cf: Measureop = tab_cf;
    //tablespoons to gallons conversion function
    let t2g: Measureop = tab_gal;
    //cubic-inches to liters conversion function
    let ci2l: Measureop = ci_lit;
    //cubic-inches to tablespoons conversion function
    let ci2t: Measureop = ci_tab;
    //cubic-inches to cups conversion function
    let ci2cps: Measureop = ci_cups;
    //cubic-inches to cubic-feet conversion function
    let ci2cf: Measureop = ci_cf;
    //cubic-inches to gallons conversion function
    let ci2g: Measureop = ci_gal;
    //cubic-feet to liters conversion function
    let cf2l: Measureop = cf_lit;
    //cubic-feet to tablespoons conversion function
    let cf2t: Measureop = cf_tab;
    //cubic-feet to cups conversion function
    let cf2cps: Measureop = cf_cups;
    //cubic-feet to cubic-inches conversion function
    let cf2ci: Measureop = cf_ci;
    //cubic-feet to gallons conversion function
    let cf2g: Measureop = cf_gal;
    //cups to liters conversion function
    let cps2l: Measureop = cups_lit;
    //cups to cubic-inches conversion function
    let cps2ci: Measureop = cups_ci;
    //cups to tablespoons conversion function
    let cps2t: Measureop = cups_tab;
    //cups to cubic-feet conversion function
    let cps2cf: Measureop = cups_cf;
    //cups to gallons conversion function
    let cps2g: Measureop = cups_gal;
    //gallons to liters conversion function
    let g2l: Measureop = gal_lit;
    //gallons to cubic-inches conversion function
    let g2ci: Measureop = gal_ci;
    //gallons to tablespoons conversion function
    let g2t: Measureop = gal_tab;
    //gallons to cubic-feet conversion function
    let g2cf: Measureop = gal_cf;
    //gallons to cups conversion function
    let g2cps: Measureop = gal_cups;

    // These are the second level hashmaps that are values
    // for the keys of the first level hashmaps.
    // The first level hasmap keys are the "uom_in" value
    let kelvin_map = hashmap![
        "CELSIUS" => k2c,
        "FAHRENHEIT" => k2f,
        "RANKINE" => k2r
    ];

    let celsius_map = hashmap![
        "KELVIN" => c2k,
        "FAHRENHEIT" => c2f,
        "RANKINE" => c2r
    ];

    let fahrenheit_map = hashmap![
        "CELSIUS" => f2c,
        "KELVIN" => f2k,
        "RANKINE" => f2r
    ];

    let rankine_map = hashmap![
        "CELSIUS" => r2c,
        "FAHRENHEIT" => r2f,
        "KELVIN" => r2k
    ];

    let liters_map = hashmap![
        "TABLESPOONS" => l2t,
        "CUBIC-INCHES" => l2ci,
        "CUBIC-FEET" => l2cf,
        "CUPS" => l2cps,
        "GALLONS" => l2g
    ];

    let tablespoons_map = hashmap![
        "LITERS" => t2l,
        "CUBIC-INCHES" => t2ci,
        "CUBIC-FEET" => t2cf,
        "CUPS" => t2cps,
        "GALLONS" => t2g
    ];

    let cubic_feet_map = hashmap![
        "LITERS" => cf2l,
        "CUBIC-INCHES" => cf2ci,
        "TABLESPOONS" => cf2t,
        "CUPS" => cf2cps,
        "GALLONS" => cf2g
    ];

    let cubic_inches_map = hashmap![
        "LITERS" => ci2l,
        "CUBIC-FEET" => ci2cf,
        "TABLESPOONS" => ci2t,
        "CUPS" => ci2cps,
        "GALLONS" => ci2g
    ];

    let cups_map = hashmap![
        "LITERS" => cps2l,
        "CUBIC-INCHES" => cps2ci,
        "TABLESPOONS" => cps2t,
        "CUBIC-FEET" => cps2cf,
        "GALLONS" => cps2g
    ];

    let gallons_map = hashmap![
        "LITERS" => g2l,
        "CUBIC-INCHES" => g2ci,
        "TABLESPOONS" => g2t,
        "CUBIC-FEET" => g2cf,
        "CUPS" => g2cps
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

    // useful in debugging
//    if true {
//	    println!("Value for uom_in: {}", args_ctx["uom_in"]);
//	    println!("Value for uom_target: {}",args_ctx["uom_target"]);
//	    println!("Value for control: {}", args_ctx["control"]);
//	    println!("Value for answer: {}", args_ctx["answer"]);
//    }

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
                    if approx_eq!(f64,r(cntrf64), ansf64,(0.05,2)){
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
