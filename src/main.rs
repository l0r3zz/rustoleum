use std::collections::HashMap;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "rustoleum", author="geoffw")]

// Struct that containes the argument parsing parameters
struct Opt {
    /// this argument holds the unit of measurement of the input value
    #[structopt(short = "i", long = "uom_in", default_value = "fahrenheit")]
    uom_in: String,
    /// this argument holds the unit of measurement of the target value
    #[structopt(short = "t", long = "uom_target", default_value = "celsius")]
    uom_target: String,
    /// The proctor control input
    control: String,
    /// The student answer
    answer: String,
}

//#[derive(debug)]

// macro to make hasmap initialization easy
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

// Function to asertain approximate equality of f64 numbers up to specified
// decimal places
fn approx_eq(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}


// List of conversion functions for all supported metrics

// Temperature conversions
// ----------------------------------------
// kelvin to celsius conversion function
fn kel_cel(n:f64) -> f64 {
    n - 273.15
}
// kelvin to fahrenheit conversion function
fn kel_fah(n:f64) -> f64 {
    (n - 273.15) * 9.0/5.0 + 32.0
}
//kelvin to rankine conversion function
fn kel_ran(n:f64) -> f64 {
    n * 1.8
}
//celsius to kelvin conversion function
fn cel_kel(n:f64) -> f64 {
    n + 273.15
}
//celsius to fahrenheit conversion function
fn cel_fah(n:f64) -> f64 {
    (n * 9.0/5.0) + 32.0
}
//celsius to rankine conversion function
fn cel_ran(n:f64) -> f64 {
    (n * 9.0/5.0) + 491.67
}
//fahrenheit to kelvin conversion function
fn fah_kel(n:f64) -> f64 {
    (n - 32.0)* 5.0/9.0 + 273.15
}
//fahrenheit to celsius conversion function
fn fah_cel(n:f64) -> f64 {
    (n - 32.0)* 5.0/9.0
}
//fahrenheit to rankine conversion function
fn fah_ran(n:f64) -> f64 {
    n  + 459.67
}
//rankine to kelvin conversion function
fn ran_kel(n:f64) -> f64 {
    (n - 32.0)* 9.0/5.0 + 273.15
}
//rankine to celsius conversion function
fn ran_cel(n:f64) -> f64 {
    (n - 32.0)* 9.0/5.0
}
//rankine to fahrenheit conversion function
fn ran_fah(n:f64) -> f64 {
    n  + 459.67
}

// Volume conversions
// ----------------------------------------
// liters to tablespoon conversion function
fn lit_tab(n:f64) -> f64 {
    n  * 67.628
}
// liters to cubic-inches conversion function
fn lit_ci(n:f64) -> f64 {
    n  * 61.023
}
// liters to cups conversion function
fn lit_cups(n:f64) -> f64 {
    n  * 4.226
}
// liters to cubic-feet conversion function
fn lit_cf(n:f64) -> f64 {
   n  * 0.0353
}
// liters to gallons conversion function
fn lit_gal(n:f64) -> f64 {
   n  * 0.2641
}

// tablespoons to liters conversion function
fn tab_lit(n:f64) -> f64 {
    n  * 0.0147
}
// tablespoons to cubic-inches conversion function
fn tab_ci(n:f64) -> f64 {
    n  * 0.902
}
// tablespoons to cups conversion function
fn tab_cups(n:f64) -> f64 {
    n  * 0.062
}
// tablespoons to cubic-feet conversion function
fn tab_cf(n:f64) -> f64 {
   n  * 0.00052219
}
// tablespoons to gallons conversion function
fn tab_gal(n:f64) -> f64 {
   n  * 0.00390625
}
// cubic-inches to liters conversion function
fn ci_lit(n:f64) -> f64 {
   n  * 0.0163
}
// cubic-inches to 0.000578704tablespoons conversion function
fn ci_tab(n:f64) -> f64 {
   n  * 1.10823
}
// cubic-inches to cups conversion function
fn ci_cups(n:f64) -> f64 {
   n  * 0.06926
}
// cubic-inches to cubic-feet conversion function
fn ci_cf(n:f64) -> f64 {
   n  * 0.000578704
}
// cubic-inches to gallons conversion function
fn ci_gal(n:f64) -> f64 {
   n  * 0.004329
}


// cups to liters conversion function
fn cups_lit(n:f64) -> f64 {
   n  * 0.236588
}

// cups to cubic-inches conversion function
fn cups_ci(n:f64) -> f64 {
   n  * 14.4375
}
// cups to tablespoons conversion function
fn cups_tab(n:f64) -> f64 {
   n  * 16.0
}
// cups to cubic-feet conversion function
fn cups_cf(n:f64) -> f64 {
   n  * 0.00835
}
// cups to gallons conversion function
fn cups_gal(n:f64) -> f64 {
   n  * 0.0625
}
// cubic-feet to liters conversion function
fn cf_lit(n:f64) -> f64 {
   n  * 28.3168
}
// cubic-feet to cubic-inches conversion function
fn cf_ci(n:f64) -> f64 {
   n  * 1728.0
}
// cubic-feet to tablespoons conversion function
fn cf_tab(n:f64) -> f64 {
   n  * 1915.01
}
// cubic-feet to cups conversion function
fn cf_cups(n:f64) -> f64 {
   n  * 119.688
}
// cubic-feet to gallons conversion function
fn cf_gal(n:f64) -> f64 {
   n  * 7.48052
}
// gallons to liters conversion function
fn gal_lit(n:f64) -> f64 {
   n  * 3.785
}
// gallons to cubic-inches conversion function
fn gal_ci(n:f64) -> f64 {
   n  * 231.0
}
// gallons to tablespoons conversion function
fn gal_tab(n:f64) -> f64 {
   n  * 256.0
}
// gallons to cubic-feet conversion function
fn gal_cf(n:f64) -> f64 {
   n  * 0.133
}
// gallons to cups conversion function
fn gal_cups(n:f64) -> f64 {
   n  * 0.0625
}



fn main() {
    let opts = Opt::from_args();
//    let uom_in = opts.uom_in;
//    let uom_target = opts.uom_target;
//    let control = opts.control;
//    let answer = opts.answer;
    let args_ctx = hashmap![
        "uom_in" => opts.uom_in.to_ascii_uppercase(),
        "uom_target" => opts.uom_target.to_ascii_uppercase(),
        "control" => opts.control,
        "answer" => opts.answer
    ];

    // function pointer type
    type Measureop = fn(f64) -> f64;

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

    // conversion maps
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

    println!("Value for uom_in: {}", args_ctx["uom_in"]);
    println!("Value for uom_target: {}",args_ctx["uom_target"]);
    println!("Value for control: {}", args_ctx["control"]);
    println!("Value for answer: {}", args_ctx["answer"]);
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
                    if approx_eq(r(cntrf64), ansf64,2){
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



// Unit test go here
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_output_result(){
        assert!(true);
    }
    #[test]
    // test Fahrenheit to Celsius functions
    fn test_fah_cel() {
        let f = 70.0;
        let c = 21.11;
        let f2c: Measureop = fah_cel;
        let res_fn = fah_cel(f);
        let res_fnp = f2c(f);
        assert!(approx_eq(res_fn, c,2));
        assert!(approx_eq(res_fnp, c,2));
    }
    #[test]
    // test Fahrenheit to kelvin functions
    fn test_fah_kel() {
        let f = 70.0;
        let k = 294.26;
        let f2k: Measureop = fah_kel;
        let res_fn = fah_kel(f);
        let res_fnp = f2k(f);
        assert!(approx_eq(res_fn, k,2));
        assert!(approx_eq(res_fnp, k,2));
    }
    #[test]
    // test Fahrenheit to Rankine functions
    fn test_fah_ran() {
        let f = 70.0;
        let r = 529.67;
        let f2r: Measureop = fah_ran;
        let res_fn = fah_ran(f);
        let res_fnp = f2r(f);
        assert!(approx_eq(res_fn, r,3));
        assert!(approx_eq(res_fnp, r,3));
    }
}
