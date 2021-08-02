use std::{error, result};
use std::env;
use std::collections::HashMap;

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


type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

//fn get_args() -> TResult<HashMap>{
fn get_args() -> TResult<String>{
    unimplemented!()
}


fn main() {
let hash_result = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
    println!("Hello, world!");
}



// Unit test go here
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_get_args() {
        use crate::get_args;
        let res = get_args();
        assert!(res.is_ok());
    }
}
