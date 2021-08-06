use std::{error, result};
use std::collections::HashMap;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "rustoleum", author="geoffw")]

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
//    for arg in env::args()

// macro to make hasmap initialization easy
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
fn approx_eq(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}


//type TResult<T> = result::Result<T, TError>;
//type TError = Box<dyn error::Error>;
// function pointer type
 type Measureop = fn(f64) -> f64;

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
    fn kel_cel(n:f64) -> f64 {
        n - 273.15
    }
    let k2c: Measureop = kel_cel;

    // kelvin to fahrenheit conversion function
    fn kel_fah(n:f64) -> f64 {
        (n - 273.15) * 9.0/5.0 + 32.0
    }
    let k2f: Measureop = kel_fah;

    //kelvin to rankine conversion function
    fn kel_ran(n:f64) -> f64 {
        n * 1.8
    }
    let k2r: Measureop = kel_ran;

    //celsius to kelvin conversion function
    fn cel_kel(n:f64) -> f64 {
        n + 273.15
    }
    let c2k: Measureop = cel_kel;


    //celsius to fahrenheit conversion function
    fn cel_fah(n:f64) -> f64 {
        (n * 9.0/5.0) + 32.0
    }
    let c2f: Measureop = cel_fah;

    //celsius to rankine conversion function
    fn cel_ran(n:f64) -> f64 {
        (n * 9.0/5.0) + 491.67
    }
    let c2r: Measureop = cel_ran;

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
        "CELSIUS" => k2c,
        "FAHRENHEIT" => k2f,
        "RANKINE" => k2r
    ];

    let rankine_map = hashmap![
        "CELSIUS" => k2c,
        "FAHRENHEIT" => k2f,
        "RANKINE" => k2r
    ];

    // Main conversion dispatch table
    let cvnmap = hashmap![
        "KELVIN" => kelvin_map,
        "CELSIUS" => celsius_map,
        "FAHRENHEIT" => fahrenheit_map,
        "RANKINE" => rankine_map
//        "LITERS" => 0,
//        "TABLESPOONS" => 0,
//        "CUBIC-INCHES" => 0,
//        "CUPS" => 0,
//        "CUBIC-FEET" => 0,
//        "GALLONS" => 0
    ];
    println!("Value for uom_in: {}", args_ctx["uom_in"]);
    println!("Value for uom_target: {}",args_ctx["uom_target"]);
    println!("Value for control: {}", args_ctx["control"]);
    println!("Value for answer: {}", args_ctx["answer"]);
    println!("Hello, world!");
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
