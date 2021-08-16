extern crate float_cmp;
//#[derive(debug)]

// List of conversion functions for all supported metrics

// Temperature conversions
// ----------------------------------------
// kelvin to celsius conversion function
// function pointer type
type Measureop = fn(f64) -> f64;

pub fn kel_cel(n:f64) -> f64 {
    n - 273.15
}
pub static K2C: Measureop = kel_cel;

// kelvin to fahrenheit conversion function
pub fn kel_fah(n:f64) -> f64 {
    (n - 273.15) * 9.0/5.0 + 32.0
}
pub static K2F: Measureop = kel_fah;

//kelvin to rankine conversion function
pub fn kel_ran(n:f64) -> f64 {
    n * 1.8
}
pub static K2R: Measureop = kel_ran;

//celsius to kelvin conversion function
pub fn cel_kel(n:f64) -> f64 {
    n + 273.15
}
pub static C2K: Measureop = cel_kel;

//celsius to fahrenheit conversion function
pub fn cel_fah(n:f64) -> f64 {
    (n * 9.0/5.0) + 32.0
}
pub static C2F: Measureop = cel_fah;

//celsius to rankine conversion function
pub fn cel_ran(n:f64) -> f64 {
    (n * 9.0/5.0) + 491.67
}
pub static C2R: Measureop = cel_ran;

//fahrenheit to kelvin conversion function
pub fn fah_kel(n:f64) -> f64 {
    (n - 32.0)* 0.5555555 + 273.15
}
pub static F2K: Measureop = fah_kel;

//fahrenheit to celsius conversion function
pub fn fah_cel(n:f64) -> f64 {
   (n - 32.0)* 0.555555
}
pub static F2C: Measureop = fah_cel;

//fahrenheit to rankine conversion function
pub fn fah_ran(n:f64) -> f64 {
    n  + 459.67
}
pub static F2R: Measureop = fah_ran;

//rankine to kelvin conversion function
pub fn ran_kel(n:f64) -> f64 {
    (n - 32.0)* 9.0/5.0 + 273.15
}
pub static R2K: Measureop = ran_kel;

//rankine to celsius conversion function
pub fn ran_cel(n:f64) -> f64 {
    (n - 32.0)* 9.0/5.0
}
pub static R2C: Measureop = ran_cel;

//rankine to fahrenheit conversion function
pub fn ran_fah(n:f64) -> f64 {
    n  + 459.67
}
pub static R2F: Measureop = ran_fah;

// Volume conversions
// ----------------------------------------
// liters to tablespoon conversion function
pub fn lit_tab(n:f64) -> f64 {
    n  * 67.628
}
pub static L2T: Measureop = lit_tab;

// liters to cubic-inches conversion function
pub fn lit_ci(n:f64) -> f64 {
    n  * 61.023
}
pub static L2CI: Measureop = lit_ci;

// liters to cups conversion function
pub fn lit_cups(n:f64) -> f64 {
    n  * 4.226
}
pub static L2CPS: Measureop = lit_cups;

// liters to cubic-feet conversion function
pub fn lit_cf(n:f64) -> f64 {
   n  * 0.0353
}
pub static L2CF: Measureop = lit_cf;

// liters to gallons conversion function
pub fn lit_gal(n:f64) -> f64 {
   n  * 0.2641
}
pub static L2G: Measureop = lit_gal;

// tablespoons to liters conversion function
pub fn tab_lit(n:f64) -> f64 {
    n  * 0.0147
}
pub static T2L: Measureop = tab_lit;

// tablespoons to cubic-inches conversion function
pub fn tab_ci(n:f64) -> f64 {
    n  * 0.902
}
pub static T2CI: Measureop = tab_ci;

// tablespoons to cups conversion function
pub fn tab_cups(n:f64) -> f64 {
    n  * 0.062
}
pub static T2CPS: Measureop = tab_cups;

// tablespoons to cubic-feet conversion function
pub fn tab_cf(n:f64) -> f64 {
   n  * 0.00052219
}
pub static T2CF: Measureop = tab_cf;

// tablespoons to gallons conversion function
pub fn tab_gal(n:f64) -> f64 {
   n  * 0.00390625
}
pub static T2G: Measureop = tab_gal;

// cubic-inches to liters conversion function
pub fn ci_lit(n:f64) -> f64 {
   n  * 0.0163
}
pub static CI2L: Measureop = ci_lit;

// cubic-inches to 0.000578704tablespoons conversion function
pub fn ci_tab(n:f64) -> f64 {
   n  * 1.10823
}
pub static CI2T: Measureop = ci_tab;

// cubic-inches to cups conversion function
pub fn ci_cups(n:f64) -> f64 {
   n  * 0.06926
}
pub static CI2CPS: Measureop = ci_cups;

// cubic-inches to cubic-feet conversion function
pub fn ci_cf(n:f64) -> f64 {
   n  * 0.000578704
}
pub static CI2CF: Measureop = ci_cf;

// cubic-inches to gallons conversion function
pub fn ci_gal(n:f64) -> f64 {
   n  * 0.004329
}
pub static CI2G: Measureop = ci_gal;

// cups to liters conversion function
pub fn cups_lit(n:f64) -> f64 {
   n  * 0.236588
}
pub static CPS2L: Measureop = cups_lit;

// cups to cubic-inches conversion function
pub fn cups_ci(n:f64) -> f64 {
   n  * 14.4375
}
pub static CPS2CI: Measureop = cups_ci;

// cups to tablespoons conversion function
pub fn cups_tab(n:f64) -> f64 {
   n  * 16.0
}
pub static CPS2T: Measureop = cups_tab;

// cups to cubic-feet conversion function
pub fn cups_cf(n:f64) -> f64 {
   n  * 0.00835
}
pub static CPS2CF: Measureop = cups_cf;

// cups to gallons conversion function
pub fn cups_gal(n:f64) -> f64 {
   n  * 0.0625
}
pub static CPS2G: Measureop = cups_gal;

// cubic-feet to liters conversion function
pub fn cf_lit(n:f64) -> f64 {
   n  * 28.3168
}
pub static CF2L: Measureop = cf_lit;

// cubic-feet to cubic-inches conversion function
pub fn cf_ci(n:f64) -> f64 {
   n  * 1728.0
}
pub static CF2CI: Measureop = cf_ci;

// cubic-feet to tablespoons conversion function
pub fn cf_tab(n:f64) -> f64 {
   n  * 1915.01
}
pub static CF2T: Measureop = cf_tab;

// cubic-feet to cups conversion function
pub fn cf_cups(n:f64) -> f64 {
   n  * 119.688
}
pub static CF2CPS: Measureop = cf_cups;

// cubic-feet to gallons conversion function
pub fn cf_gal(n:f64) -> f64 {
   n  * 7.48052
}
pub static CF2G: Measureop = cf_gal;

// gallons to liters conversion function
pub fn gal_lit(n:f64) -> f64 {
   n  * 3.785
}
pub static G2L: Measureop = gal_lit;

// gallons to cubic-inches conversion function
pub fn gal_ci(n:f64) -> f64 {
   n  * 231.0
}
pub static G2CI: Measureop = gal_ci;
// gallons to tablespoons conversion function
pub fn gal_tab(n:f64) -> f64 {
   n  * 256.0
}
pub static G2T: Measureop = gal_tab;

// gallons to cubic-feet conversion function
pub fn gal_cf(n:f64) -> f64 {
   n  * 0.133
}
pub static G2CF: Measureop = gal_cf;

// gallons to cups conversion function
pub fn gal_cups(n:f64) -> f64 {
   n  * 0.0625
}
pub static G2CPS: Measureop = gal_cups;


// Unit test go here
#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;

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
        let res_fn = fah_cel(f);
        let res_fnp = F2C(f);
        assert!(approx_eq!(f64,res_fn, c,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, c,(0.005,2)));
    }
    #[test]
    // test Fahrenheit to kelvin functions
    fn test_fah_kel() {
        let f = 70.0;
        let k = 294.26;
        let res_fn = fah_kel(f);
        let res_fnp = F2K(f);
        assert!(approx_eq!(f64,res_fn, k,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, k,(0.005,2)));
    }
    #[test]
    // test Fahrenheit to Rankine functions
    fn test_fah_ran() {
        let f = 70.00;
        let r = 529.67;
        let res_fn = fah_ran(f);
        let res_fnp = F2R(f);
        assert!(approx_eq!(f64,res_fn, r,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, r,(0.005,2)));
    }
    #[test]
    // test Celsius to Kelvin functions
    fn test_cel_kel() {
        let c = 70.0;
        let k = 343.15;
        let res_fn = cel_kel(c);
        let res_fnp = C2K(c);
        assert!(approx_eq!(f64,res_fn, k,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, k,(0.005,2)));
    }
    #[test]
    // test Celsius to Fahrenheit functions
    fn test_cel_fah() {
        let c = 70.0;
        let f = 158.0;
        let res_fn = cel_fah(c);
        let res_fnp = C2F(c);
        assert!(approx_eq!(f64,res_fn, f,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, f,(0.005,2)));
    }
    #[test]
    // test Celsius to Rankine functions
    fn test_cel_ran() {
        let c = 70.0;
        let r = 617.67;
        let res_fn = cel_ran(c);
        let res_fnp = C2R(c);
        assert!(approx_eq!(f64,res_fn, r,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, r,(0.005,2)));
    }
    #[test]
    // test Kelvin to Celsius functions
    fn test_kel_cel() {
        let k = 70.0;
        let c = -203.15;
        let res_fn = kel_cel(k);
        let res_fnp = K2C(k);
        assert!(approx_eq!(f64,res_fn, c,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, c,(0.005,2)));
    }
    #[test]
    // test Kelvin to Fahrenheit functions
    fn test_kel_fah() {
        let k = 70.0;
        let f = -333.67;
        let res_fn = kel_fah(k);
        let res_fnp = K2F(k);
        assert!(approx_eq!(f64,res_fn, f,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, f,(0.005,2)));
    }
    #[test]
    // test Kelvin to Rankine functions
    fn test_kel_ran() {
        let k = 70.0;
        let r = 126.0;
        let res_fn = kel_ran(k);
        let res_fnp = K2R(k);
        assert!(approx_eq!(f64,res_fn, r,(0.005,2)));
        assert!(approx_eq!(f64,res_fnp, r,(0.005,2)));
    }
}
