use std::{error, result};
use std::collections::HashMap;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "rustoleum")]

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
//macro_rules! hashmap {
//    ($( $key: expr => $val: expr ),*) => {{
//         let mut map = HashMap::new();
//         $( map.insert($key, $val); )*
//         map
//    }}
//}


//type TResult<T> = result::Result<T, TError>;
//type TError = Box<dyn error::Error>;


fn main() {
    let opts = Opt::from_args();
    let uom_in = opts.uom_in;
    let uom_target = opts.uom_target;
    let control = opts.control;
    let answer = opts.answer;
    println!("Value for uom_in: {}", uom_in.to_ascii_uppercase());
    println!("Value for uom_target: {}", uom_target.to_ascii_uppercase());
    println!("Value for control: {}", control);
    println!("Value for answer: {}", answer);
    println!("Hello, world!");
}



// Unit test go here
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
