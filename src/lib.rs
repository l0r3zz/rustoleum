extern crate float_cmp;
//#[derive(debug)]

// Function to asertain approximate equality of f64 numbers up to specified
// decimal places
//pub fn approx_eq(a: f64, b: f64, decimal_places: u8) -> bool {
//    let factor = 10.0f64.powi(decimal_places as i32);
//    let a = (a * factor).trunc();
//    let b = (b * factor).trunc();
//    a == b
//}

// List of conversion functions for all supported metrics

// Temperature conversions
// ----------------------------------------
// kelvin to celsius conversion function
pub fn kel_cel(n:f64) -> f64 {
    n - 273.15
}
// kelvin to fahrenheit conversion function
pub fn kel_fah(n:f64) -> f64 {
    (n - 273.15) * 9.0/5.0 + 32.0
}
//kelvin to rankine conversion function
pub fn kel_ran(n:f64) -> f64 {
    n * 1.8
}
//celsius to kelvin conversion function
pub fn cel_kel(n:f64) -> f64 {
    n + 273.15
}
//celsius to fahrenheit conversion function
pub fn cel_fah(n:f64) -> f64 {
    (n * 9.0/5.0) + 32.0
}
//celsius to rankine conversion function
pub fn cel_ran(n:f64) -> f64 {
    (n * 9.0/5.0) + 491.67
}
//fahrenheit to kelvin conversion function
pub fn fah_kel(n:f64) -> f64 {
    (n - 32.0)* 0.5555555 + 273.15
}
//fahrenheit to celsius conversion function
pub fn fah_cel(n:f64) -> f64 {
   (n - 32.0)* 0.555555
}
//fahrenheit to rankine conversion function
pub fn fah_ran(n:f64) -> f64 {
    n  + 459.67
}
//rankine to kelvin conversion function
pub fn ran_kel(n:f64) -> f64 {
    (n - 32.0)* 9.0/5.0 + 273.15
}
//rankine to celsius conversion function
pub fn ran_cel(n:f64) -> f64 {
    (n - 32.0)* 9.0/5.0
}
//rankine to fahrenheit conversion function
pub fn ran_fah(n:f64) -> f64 {
    n  + 459.67
}

// Volume conversions
// ----------------------------------------
// liters to tablespoon conversion function
pub fn lit_tab(n:f64) -> f64 {
    n  * 67.628
}
// liters to cubic-inches conversion function
pub fn lit_ci(n:f64) -> f64 {
    n  * 61.023
}
// liters to cups conversion function
pub fn lit_cups(n:f64) -> f64 {
    n  * 4.226
}
// liters to cubic-feet conversion function
pub fn lit_cf(n:f64) -> f64 {
   n  * 0.0353
}
// liters to gallons conversion function
pub fn lit_gal(n:f64) -> f64 {
   n  * 0.2641
}

// tablespoons to liters conversion function
pub fn tab_lit(n:f64) -> f64 {
    n  * 0.0147
}
// tablespoons to cubic-inches conversion function
pub fn tab_ci(n:f64) -> f64 {
    n  * 0.902
}
// tablespoons to cups conversion function
pub fn tab_cups(n:f64) -> f64 {
    n  * 0.062
}
// tablespoons to cubic-feet conversion function
pub fn tab_cf(n:f64) -> f64 {
   n  * 0.00052219
}
// tablespoons to gallons conversion function
pub fn tab_gal(n:f64) -> f64 {
   n  * 0.00390625
}
// cubic-inches to liters conversion function
pub fn ci_lit(n:f64) -> f64 {
   n  * 0.0163
}
// cubic-inches to 0.000578704tablespoons conversion function
pub fn ci_tab(n:f64) -> f64 {
   n  * 1.10823
}
// cubic-inches to cups conversion function
pub fn ci_cups(n:f64) -> f64 {
   n  * 0.06926
}
// cubic-inches to cubic-feet conversion function
pub fn ci_cf(n:f64) -> f64 {
   n  * 0.000578704
}
// cubic-inches to gallons conversion function
pub fn ci_gal(n:f64) -> f64 {
   n  * 0.004329
}
// cups to liters conversion function
pub fn cups_lit(n:f64) -> f64 {
   n  * 0.236588
}

// cups to cubic-inches conversion function
pub fn cups_ci(n:f64) -> f64 {
   n  * 14.4375
}
// cups to tablespoons conversion function
pub fn cups_tab(n:f64) -> f64 {
   n  * 16.0
}
// cups to cubic-feet conversion function
pub fn cups_cf(n:f64) -> f64 {
   n  * 0.00835
}
// cups to gallons conversion function
pub fn cups_gal(n:f64) -> f64 {
   n  * 0.0625
}
// cubic-feet to liters conversion function
pub fn cf_lit(n:f64) -> f64 {
   n  * 28.3168
}
// cubic-feet to cubic-inches conversion function
pub fn cf_ci(n:f64) -> f64 {
   n  * 1728.0
}
// cubic-feet to tablespoons conversion function
pub fn cf_tab(n:f64) -> f64 {
   n  * 1915.01
}
// cubic-feet to cups conversion function
pub fn cf_cups(n:f64) -> f64 {
   n  * 119.688
}
// cubic-feet to gallons conversion function
pub fn cf_gal(n:f64) -> f64 {
   n  * 7.48052
}
// gallons to liters conversion function
pub fn gal_lit(n:f64) -> f64 {
   n  * 3.785
}
// gallons to cubic-inches conversion function
pub fn gal_ci(n:f64) -> f64 {
   n  * 231.0
}
// gallons to tablespoons conversion function
pub fn gal_tab(n:f64) -> f64 {
   n  * 256.0
}
// gallons to cubic-feet conversion function
pub fn gal_cf(n:f64) -> f64 {
   n  * 0.133
}
// gallons to cups conversion function
pub fn gal_cups(n:f64) -> f64 {
   n  * 0.0625
}


// Unit test go here
#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;
     // function pointer type
    type Measureop = fn(f64) -> f64;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_approx_eq() {
        assert!( approx_eq!(f64,9.0/11.0, 0.8182,(0.005,2)));
    }

    #[test]
    // test Fahrenheit to Celsius functions
    fn test_fah_cel() {
        let f = 70.0;
        let c = 21.11;
        let f2c: Measureop = fah_cel;
        let res_fn = fah_cel(f);
        let res_fnp = f2c(f);
        assert!(approx_eq!(f64,res_fn, c,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, c,(0.005,2)));
    }
    #[test]
    // test Fahrenheit to kelvin functions
    fn test_fah_kel() {
        let f = 70.0;
        let k = 294.26;
        let f2k: Measureop = fah_kel;
        let res_fn = fah_kel(f);
        let res_fnp = f2k(f);
        assert!(approx_eq!(f64,res_fn, k,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, k,(0.005,2)));
    }
    #[test]
    // test Fahrenheit to Rankine functions
    fn test_fah_ran() {
        let f = 70.00;
        let r = 529.67;
        let f2r: Measureop = fah_ran;
        let res_fn = fah_ran(f);
        let res_fnp = f2r(f);
        assert!(approx_eq!(f64,res_fn, r,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, r,(0.005,2)));
    }
    #[test]
    // test Celsius to Kelvin functions
    fn test_cel_kel() {
        let c = 70.0;
        let k = 343.15;
        let c2k: Measureop = cel_kel;
        let res_fn = cel_kel(c);
        let res_fnp = c2k(c);
        assert!(approx_eq!(f64,res_fn, k,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, k,(0.005,2)));
    }
    #[test]
    // test Celsius to Fahrenheit functions
    fn test_cel_fah() {
        let c = 70.0;
        let f = 158.0;
        let c2f: Measureop = cel_fah;
        let res_fn = cel_fah(c);
        let res_fnp = c2f(c);
        assert!(approx_eq!(f64,res_fn, f,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, f,(0.005,2)));
    }
    #[test]
    // test Celsius to Rankine functions
    fn test_cel_ran() {
        let c = 70.0;
        let r = 617.67;
        let c2r: Measureop = cel_ran;
        let res_fn = cel_ran(c);
        let res_fnp = c2r(c);
        assert!(approx_eq!(f64,res_fn, r,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, r,(0.005,2)));
    }
    #[test]
    // test Kelvin to Celsius functions
    fn test_kel_cel() {
        let k = 70.0;
        let c = -203.15;
        let k2c: Measureop = kel_cel;
        let res_fn = kel_cel(k);
        let res_fnp = k2c(k);
        assert!(approx_eq!(f64,res_fn, c,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, c,(0.005,2)));
    }
    #[test]
    // test Kelvin to Fahrenheit functions
    fn test_kel_fah() {
        let k = 70.0;
        let f = -333.67;
        let k2f: Measureop = kel_fah;
        let res_fn = kel_fah(k);
        let res_fnp = k2f(k);
        assert!(approx_eq!(f64,res_fn, f,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, f,(0.005,2)));
    }
    #[test]
    // test Kelvin to Rankine functions
    fn test_kel_ran() {
        let k = 70.0;
        let r = 126.0;
        let k2r: Measureop = kel_ran;
        let res_fn = kel_ran(k);
        let res_fnp = k2r(k);
        assert!(approx_eq!(f64,res_fn, r,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, r,(0.005,2)));
    }
}
