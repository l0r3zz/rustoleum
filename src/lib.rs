//! # Rustoleum
//!
//! A library for converting between temperature and volume units.
//!
//! This library provides type-safe unit conversions for:
//! - **Temperature units**: Kelvin, Celsius, Fahrenheit, and Rankine
//! - **Volume units**: Liters, Tablespoons, Cubic Inches, Cups, Cubic Feet, and Gallons
//!
//! ## Example
//!
//! ```rust
//! use rustoleum::{Unit, convert};
//!
//! // Convert 70 degrees Celsius to Kelvin
//! let result = convert(Unit::Celsius, Unit::Kelvin, 70.0);
//! assert_eq!(result, Some(343.15));
//!
//! // Convert 1 liter to gallons
//! let result = convert(Unit::Liters, Unit::Gallons, 1.0);
//! assert_eq!(result, Some(0.2641));
//! ```

use std::str::FromStr;
use std::fmt;

/// Epsilon value for floating-point comparison tolerance.
///
/// Used with `float_cmp::approx_eq!` to determine if two floating-point values
/// are approximately equal. This is the default tolerance for single conversions.
pub const TOLERANCE_EPSILON: f64 = 0.005;

/// ULPS (Units in the Last Place) value for floating-point comparison tolerance.
///
/// Used with `float_cmp::approx_eq!` to determine if two floating-point values
/// are approximately equal. This is the default tolerance for single conversions.
pub const TOLERANCE_ULPS: i64 = 2;

/// Epsilon value for round-trip conversion tolerance.
///
/// Round-trip conversions (e.g., A → B → A) accumulate error, so they require
/// a larger tolerance than single conversions.
pub const ROUNDTRIP_TOLERANCE_EPSILON: f64 = 0.1;

/// ULPS value for round-trip conversion tolerance.
///
/// Round-trip conversions (e.g., A → B → A) accumulate error, so they require
/// a larger tolerance than single conversions.
pub const ROUNDTRIP_TOLERANCE_ULPS: i64 = 2;

// Temperature conversion constants
pub const ABSOLUTE_ZERO_CELSIUS: f64 = 273.15;
pub const FAHRENHEIT_FREEZING: f64 = 32.0;
pub const FAHRENHEIT_TO_RANKINE_OFFSET: f64 = 459.67;
pub const CELSIUS_TO_RANKINE_OFFSET: f64 = 491.67;
pub const CELSIUS_TO_FAHRENHEIT_RATIO: f64 = 9.0 / 5.0;
pub const FAHRENHEIT_TO_CELSIUS_RATIO: f64 = 5.0 / 9.0;
pub const KELVIN_TO_RANKINE_RATIO: f64 = 1.8;
pub const RANKINE_TO_KELVIN_RATIO: f64 = 5.0 / 9.0;

// Volume conversion constants
pub const LITERS_TO_TABLESPOONS: f64 = 67.628;
pub const LITERS_TO_CUBIC_INCHES: f64 = 61.023;
pub const LITERS_TO_CUPS: f64 = 4.226;
pub const LITERS_TO_CUBIC_FEET: f64 = 0.0353;
pub const LITERS_TO_GALLONS: f64 = 0.2641;
pub const TABLESPOONS_TO_LITERS: f64 = 0.0147;
pub const TABLESPOONS_TO_CUBIC_INCHES: f64 = 0.902;
pub const TABLESPOONS_TO_CUPS: f64 = 0.062;
pub const TABLESPOONS_TO_CUBIC_FEET: f64 = 0.00052219;
pub const TABLESPOONS_TO_GALLONS: f64 = 0.00390625;
pub const CUBIC_INCHES_TO_LITERS: f64 = 0.0163;
pub const CUBIC_INCHES_TO_TABLESPOONS: f64 = 1.10823;
pub const CUBIC_INCHES_TO_CUPS: f64 = 0.06926;
pub const CUBIC_INCHES_TO_CUBIC_FEET: f64 = 0.000578704;
pub const CUBIC_INCHES_TO_GALLONS: f64 = 0.004329;
pub const CUPS_TO_LITERS: f64 = 0.236588;
pub const CUPS_TO_CUBIC_INCHES: f64 = 14.4375;
pub const CUPS_TO_TABLESPOONS: f64 = 16.0;
pub const CUPS_TO_CUBIC_FEET: f64 = 0.00835;
pub const CUPS_TO_GALLONS: f64 = 0.0625;
pub const CUBIC_FEET_TO_LITERS: f64 = 28.3168;
pub const CUBIC_FEET_TO_CUBIC_INCHES: f64 = 1728.0;
pub const CUBIC_FEET_TO_TABLESPOONS: f64 = 1915.01;
pub const CUBIC_FEET_TO_CUPS: f64 = 119.688;
pub const CUBIC_FEET_TO_GALLONS: f64 = 7.48052;
pub const GALLONS_TO_LITERS: f64 = 3.785;
pub const GALLONS_TO_CUBIC_INCHES: f64 = 231.0;
pub const GALLONS_TO_TABLESPOONS: f64 = 256.0;
pub const GALLONS_TO_CUBIC_FEET: f64 = 0.133;
pub const GALLONS_TO_CUPS: f64 = 16.0;

/// Error type for unit parsing operations.
///
/// This error is returned when attempting to parse a string into a [`Unit`]
/// and the string does not match any known unit.
///
/// # Example
///
/// ```rust
/// use rustoleum::{Unit, UnitParseError};
/// use std::str::FromStr;
///
/// let result = Unit::from_str("invalid");
/// assert!(matches!(result, Err(UnitParseError::UnknownUnit(_))));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitParseError {
    /// The provided string does not match any known unit.
    UnknownUnit(String),
}

impl fmt::Display for UnitParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitParseError::UnknownUnit(unit) => {
                write!(f, "Unknown unit: '{}'", unit)
            }
        }
    }
}

impl std::error::Error for UnitParseError {}

/// Type-safe representation of measurement units.
///
/// This enum represents all supported units for temperature and volume conversions.
/// It can be parsed from strings (case-insensitive) and used with the [`convert`] function.
///
/// # Supported Units
///
/// ## Temperature Units
/// - `Kelvin` - Absolute temperature scale
/// - `Celsius` - Metric temperature scale
/// - `Fahrenheit` - Imperial temperature scale
/// - `Rankine` - Absolute temperature scale (Fahrenheit-based)
///
/// ## Volume Units
/// - `Liters` - Metric volume unit
/// - `Tablespoons` - US customary volume unit
/// - `CubicInches` - Imperial volume unit (also accepts "CUBIC-INCHES" or "CUBICINCHES")
/// - `Cups` - US customary volume unit
/// - `CubicFeet` - Imperial volume unit (also accepts "CUBIC-FEET" or "CUBICFEET")
/// - `Gallons` - US customary volume unit
///
/// # Example
///
/// ```rust
/// use rustoleum::Unit;
/// use std::str::FromStr;
///
/// // Parse from string (case-insensitive)
/// let unit = Unit::from_str("celsius").unwrap();
/// assert_eq!(unit, Unit::Celsius);
///
/// // Use in conversions
/// use rustoleum::convert;
/// let result = convert(Unit::Celsius, Unit::Fahrenheit, 100.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Kelvin - Absolute temperature scale (0 K = absolute zero)
    Kelvin,
    /// Celsius - Metric temperature scale (0°C = freezing point of water)
    Celsius,
    /// Fahrenheit - Imperial temperature scale (32°F = freezing point of water)
    Fahrenheit,
    /// Rankine - Absolute temperature scale based on Fahrenheit (0°R = absolute zero)
    Rankine,
    /// Liters - Metric volume unit
    Liters,
    /// Tablespoons - US customary volume unit
    Tablespoons,
    /// Cubic Inches - Imperial volume unit
    CubicInches,
    /// Cups - US customary volume unit
    Cups,
    /// Cubic Feet - Imperial volume unit
    CubicFeet,
    /// Gallons - US customary volume unit
    Gallons,
}

impl FromStr for Unit {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "KELVIN" => Ok(Unit::Kelvin),
            "CELSIUS" => Ok(Unit::Celsius),
            "FAHRENHEIT" => Ok(Unit::Fahrenheit),
            "RANKINE" => Ok(Unit::Rankine),
            "LITERS" => Ok(Unit::Liters),
            "TABLESPOONS" => Ok(Unit::Tablespoons),
            "CUBIC-INCHES" | "CUBICINCHES" => Ok(Unit::CubicInches),
            "CUPS" => Ok(Unit::Cups),
            "CUBIC-FEET" | "CUBICFEET" => Ok(Unit::CubicFeet),
            "GALLONS" => Ok(Unit::Gallons),
            _ => Err(UnitParseError::UnknownUnit(s.to_string())),
        }
    }
}

/// Converts a value from one unit to another.
///
/// This function performs type-safe unit conversions between compatible units.
/// Temperature units can only be converted to other temperature units, and
/// volume units can only be converted to other volume units.
///
/// # Arguments
///
/// * `from` - The source unit to convert from
/// * `to` - The target unit to convert to
/// * `value` - The numeric value to convert
///
/// # Returns
///
/// * `Some(result)` - The converted value if the conversion is valid
/// * `None` - If the conversion is invalid (e.g., temperature to volume)
///
/// # Examples
///
/// ```rust
/// use rustoleum::{Unit, convert};
///
/// // Convert 70 degrees Celsius to Kelvin
/// let result = convert(Unit::Celsius, Unit::Kelvin, 70.0);
/// assert_eq!(result, Some(343.15));
///
/// // Convert 1 liter to gallons
/// let result = convert(Unit::Liters, Unit::Gallons, 1.0);
/// assert_eq!(result, Some(0.2641));
///
/// // Same unit returns the same value
/// let result = convert(Unit::Celsius, Unit::Celsius, 100.0);
/// assert_eq!(result, Some(100.0));
///
/// // Invalid conversion (temperature to volume) returns None
/// let result = convert(Unit::Celsius, Unit::Liters, 100.0);
/// assert_eq!(result, None);
/// ```
pub fn convert(from: Unit, to: Unit, value: f64) -> Option<f64> {
    if from == to {
        return Some(value);
    }

    match (from, to) {
        // Temperature conversions
        (Unit::Kelvin, Unit::Celsius) => Some(kel_cel(value)),
        (Unit::Kelvin, Unit::Fahrenheit) => Some(kel_fah(value)),
        (Unit::Kelvin, Unit::Rankine) => Some(kel_ran(value)),
        (Unit::Celsius, Unit::Kelvin) => Some(cel_kel(value)),
        (Unit::Celsius, Unit::Fahrenheit) => Some(cel_fah(value)),
        (Unit::Celsius, Unit::Rankine) => Some(cel_ran(value)),
        (Unit::Fahrenheit, Unit::Kelvin) => Some(fah_kel(value)),
        (Unit::Fahrenheit, Unit::Celsius) => Some(fah_cel(value)),
        (Unit::Fahrenheit, Unit::Rankine) => Some(fah_ran(value)),
        (Unit::Rankine, Unit::Kelvin) => Some(ran_kel(value)),
        (Unit::Rankine, Unit::Celsius) => Some(ran_cel(value)),
        (Unit::Rankine, Unit::Fahrenheit) => Some(ran_fah(value)),
        // Volume conversions
        (Unit::Liters, Unit::Tablespoons) => Some(lit_tab(value)),
        (Unit::Liters, Unit::CubicInches) => Some(lit_ci(value)),
        (Unit::Liters, Unit::Cups) => Some(lit_cups(value)),
        (Unit::Liters, Unit::CubicFeet) => Some(lit_cf(value)),
        (Unit::Liters, Unit::Gallons) => Some(lit_gal(value)),
        (Unit::Tablespoons, Unit::Liters) => Some(tab_lit(value)),
        (Unit::Tablespoons, Unit::CubicInches) => Some(tab_ci(value)),
        (Unit::Tablespoons, Unit::Cups) => Some(tab_cups(value)),
        (Unit::Tablespoons, Unit::CubicFeet) => Some(tab_cf(value)),
        (Unit::Tablespoons, Unit::Gallons) => Some(tab_gal(value)),
        (Unit::CubicInches, Unit::Liters) => Some(ci_lit(value)),
        (Unit::CubicInches, Unit::Tablespoons) => Some(ci_tab(value)),
        (Unit::CubicInches, Unit::Cups) => Some(ci_cups(value)),
        (Unit::CubicInches, Unit::CubicFeet) => Some(ci_cf(value)),
        (Unit::CubicInches, Unit::Gallons) => Some(ci_gal(value)),
        (Unit::Cups, Unit::Liters) => Some(cups_lit(value)),
        (Unit::Cups, Unit::CubicInches) => Some(cups_ci(value)),
        (Unit::Cups, Unit::Tablespoons) => Some(cups_tab(value)),
        (Unit::Cups, Unit::CubicFeet) => Some(cups_cf(value)),
        (Unit::Cups, Unit::Gallons) => Some(cups_gal(value)),
        (Unit::CubicFeet, Unit::Liters) => Some(cf_lit(value)),
        (Unit::CubicFeet, Unit::CubicInches) => Some(cf_ci(value)),
        (Unit::CubicFeet, Unit::Tablespoons) => Some(cf_tab(value)),
        (Unit::CubicFeet, Unit::Cups) => Some(cf_cups(value)),
        (Unit::CubicFeet, Unit::Gallons) => Some(cf_gal(value)),
        (Unit::Gallons, Unit::Liters) => Some(gal_lit(value)),
        (Unit::Gallons, Unit::CubicInches) => Some(gal_ci(value)),
        (Unit::Gallons, Unit::Tablespoons) => Some(gal_tab(value)),
        (Unit::Gallons, Unit::CubicFeet) => Some(gal_cf(value)),
        (Unit::Gallons, Unit::Cups) => Some(gal_cups(value)),
        // Invalid conversions (temperature to volume or vice versa)
        _ => None,
    }
}

// Temperature conversions
// ----------------------------------------

/// Converts Kelvin to Celsius.
///
/// # Arguments
///
/// * `n` - Temperature in Kelvin
///
/// # Returns
///
/// Temperature in Celsius
///
/// # Example
///
/// ```rust
/// use rustoleum::kel_cel;
///
/// let celsius = kel_cel(273.15);
/// assert_eq!(celsius, 0.0);
/// ```
pub fn kel_cel(n: f64) -> f64 {
    n - ABSOLUTE_ZERO_CELSIUS
}

/// Converts Kelvin to Fahrenheit.
///
/// # Arguments
///
/// * `n` - Temperature in Kelvin
///
/// # Returns
///
/// Temperature in Fahrenheit
pub fn kel_fah(n: f64) -> f64 {
    (n - ABSOLUTE_ZERO_CELSIUS) * CELSIUS_TO_FAHRENHEIT_RATIO + FAHRENHEIT_FREEZING
}

/// Converts Kelvin to Rankine.
///
/// # Arguments
///
/// * `n` - Temperature in Kelvin
///
/// # Returns
///
/// Temperature in Rankine
pub fn kel_ran(n: f64) -> f64 {
    n * KELVIN_TO_RANKINE_RATIO
}

/// Converts Celsius to Kelvin.
///
/// # Arguments
///
/// * `n` - Temperature in Celsius
///
/// # Returns
///
/// Temperature in Kelvin
///
/// # Example
///
/// ```rust
/// use rustoleum::cel_kel;
///
/// let kelvin = cel_kel(0.0);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn cel_kel(n: f64) -> f64 {
    n + ABSOLUTE_ZERO_CELSIUS
}

/// Converts Celsius to Fahrenheit.
///
/// # Arguments
///
/// * `n` - Temperature in Celsius
///
/// # Returns
///
/// Temperature in Fahrenheit
///
/// # Example
///
/// ```rust
/// use rustoleum::cel_fah;
///
/// let fahrenheit = cel_fah(0.0);
/// assert_eq!(fahrenheit, 32.0);
/// ```
pub fn cel_fah(n: f64) -> f64 {
    (n * CELSIUS_TO_FAHRENHEIT_RATIO) + FAHRENHEIT_FREEZING
}

/// Converts Celsius to Rankine.
///
/// # Arguments
///
/// * `n` - Temperature in Celsius
///
/// # Returns
///
/// Temperature in Rankine
pub fn cel_ran(n: f64) -> f64 {
    (n * CELSIUS_TO_FAHRENHEIT_RATIO) + CELSIUS_TO_RANKINE_OFFSET
}

/// Converts Fahrenheit to Kelvin.
///
/// # Arguments
///
/// * `n` - Temperature in Fahrenheit
///
/// # Returns
///
/// Temperature in Kelvin
pub fn fah_kel(n: f64) -> f64 {
    (n - FAHRENHEIT_FREEZING) * FAHRENHEIT_TO_CELSIUS_RATIO + ABSOLUTE_ZERO_CELSIUS
}

/// Converts Fahrenheit to Celsius.
///
/// # Arguments
///
/// * `n` - Temperature in Fahrenheit
///
/// # Returns
///
/// Temperature in Celsius
///
/// # Example
///
/// ```rust
/// use rustoleum::fah_cel;
///
/// let celsius = fah_cel(32.0);
/// assert_eq!(celsius, 0.0);
/// ```
pub fn fah_cel(n: f64) -> f64 {
    (n - FAHRENHEIT_FREEZING) * FAHRENHEIT_TO_CELSIUS_RATIO
}

/// Converts Fahrenheit to Rankine.
///
/// # Arguments
///
/// * `n` - Temperature in Fahrenheit
///
/// # Returns
///
/// Temperature in Rankine
pub fn fah_ran(n: f64) -> f64 {
    n + FAHRENHEIT_TO_RANKINE_OFFSET
}

/// Converts Rankine to Kelvin.
///
/// # Arguments
///
/// * `n` - Temperature in Rankine
///
/// # Returns
///
/// Temperature in Kelvin
pub fn ran_kel(n: f64) -> f64 {
    n * RANKINE_TO_KELVIN_RATIO
}

/// Converts Rankine to Celsius.
///
/// # Arguments
///
/// * `n` - Temperature in Rankine
///
/// # Returns
///
/// Temperature in Celsius
pub fn ran_cel(n: f64) -> f64 {
    (n * RANKINE_TO_KELVIN_RATIO) - ABSOLUTE_ZERO_CELSIUS
}

/// Converts Rankine to Fahrenheit.
///
/// # Arguments
///
/// * `n` - Temperature in Rankine
///
/// # Returns
///
/// Temperature in Fahrenheit
pub fn ran_fah(n: f64) -> f64 {
    n - FAHRENHEIT_TO_RANKINE_OFFSET
}

// Volume conversions
// ----------------------------------------

/// Converts Liters to Tablespoons.
///
/// # Arguments
///
/// * `n` - Volume in liters
///
/// # Returns
///
/// Volume in tablespoons
pub fn lit_tab(n: f64) -> f64 {
    n * LITERS_TO_TABLESPOONS
}

/// Converts Liters to Cubic Inches.
///
/// # Arguments
///
/// * `n` - Volume in liters
///
/// # Returns
///
/// Volume in cubic inches
pub fn lit_ci(n: f64) -> f64 {
    n * LITERS_TO_CUBIC_INCHES
}

/// Converts Liters to Cups.
///
/// # Arguments
///
/// * `n` - Volume in liters
///
/// # Returns
///
/// Volume in cups
pub fn lit_cups(n: f64) -> f64 {
    n * LITERS_TO_CUPS
}

/// Converts Liters to Cubic Feet.
///
/// # Arguments
///
/// * `n` - Volume in liters
///
/// # Returns
///
/// Volume in cubic feet
pub fn lit_cf(n: f64) -> f64 {
    n * LITERS_TO_CUBIC_FEET
}

/// Converts Liters to Gallons.
///
/// # Arguments
///
/// * `n` - Volume in liters
///
/// # Returns
///
/// Volume in gallons
pub fn lit_gal(n: f64) -> f64 {
    n * LITERS_TO_GALLONS
}

/// Converts Tablespoons to Liters.
///
/// # Arguments
///
/// * `n` - Volume in tablespoons
///
/// # Returns
///
/// Volume in liters
pub fn tab_lit(n: f64) -> f64 {
    n * TABLESPOONS_TO_LITERS
}

/// Converts Tablespoons to Cubic Inches.
///
/// # Arguments
///
/// * `n` - Volume in tablespoons
///
/// # Returns
///
/// Volume in cubic inches
pub fn tab_ci(n: f64) -> f64 {
    n * TABLESPOONS_TO_CUBIC_INCHES
}

/// Converts Tablespoons to Cups.
///
/// # Arguments
///
/// * `n` - Volume in tablespoons
///
/// # Returns
///
/// Volume in cups
pub fn tab_cups(n: f64) -> f64 {
    n * TABLESPOONS_TO_CUPS
}

/// Converts Tablespoons to Cubic Feet.
///
/// # Arguments
///
/// * `n` - Volume in tablespoons
///
/// # Returns
///
/// Volume in cubic feet
pub fn tab_cf(n: f64) -> f64 {
    n * TABLESPOONS_TO_CUBIC_FEET
}

/// Converts Tablespoons to Gallons.
///
/// # Arguments
///
/// * `n` - Volume in tablespoons
///
/// # Returns
///
/// Volume in gallons
pub fn tab_gal(n: f64) -> f64 {
    n * TABLESPOONS_TO_GALLONS
}

/// Converts Cubic Inches to Liters.
///
/// # Arguments
///
/// * `n` - Volume in cubic inches
///
/// # Returns
///
/// Volume in liters
pub fn ci_lit(n: f64) -> f64 {
    n * CUBIC_INCHES_TO_LITERS
}

/// Converts Cubic Inches to Tablespoons.
///
/// # Arguments
///
/// * `n` - Volume in cubic inches
///
/// # Returns
///
/// Volume in tablespoons
pub fn ci_tab(n: f64) -> f64 {
    n * CUBIC_INCHES_TO_TABLESPOONS
}

/// Converts Cubic Inches to Cups.
///
/// # Arguments
///
/// * `n` - Volume in cubic inches
///
/// # Returns
///
/// Volume in cups
pub fn ci_cups(n: f64) -> f64 {
    n * CUBIC_INCHES_TO_CUPS
}

/// Converts Cubic Inches to Cubic Feet.
///
/// # Arguments
///
/// * `n` - Volume in cubic inches
///
/// # Returns
///
/// Volume in cubic feet
pub fn ci_cf(n: f64) -> f64 {
    n * CUBIC_INCHES_TO_CUBIC_FEET
}

/// Converts Cubic Inches to Gallons.
///
/// # Arguments
///
/// * `n` - Volume in cubic inches
///
/// # Returns
///
/// Volume in gallons
pub fn ci_gal(n: f64) -> f64 {
    n * CUBIC_INCHES_TO_GALLONS
}

/// Converts Cups to Liters.
///
/// # Arguments
///
/// * `n` - Volume in cups
///
/// # Returns
///
/// Volume in liters
pub fn cups_lit(n: f64) -> f64 {
    n * CUPS_TO_LITERS
}

/// Converts Cups to Cubic Inches.
///
/// # Arguments
///
/// * `n` - Volume in cups
///
/// # Returns
///
/// Volume in cubic inches
pub fn cups_ci(n: f64) -> f64 {
    n * CUPS_TO_CUBIC_INCHES
}

/// Converts Cups to Tablespoons.
///
/// # Arguments
///
/// * `n` - Volume in cups
///
/// # Returns
///
/// Volume in tablespoons
pub fn cups_tab(n: f64) -> f64 {
    n * CUPS_TO_TABLESPOONS
}

/// Converts Cups to Cubic Feet.
///
/// # Arguments
///
/// * `n` - Volume in cups
///
/// # Returns
///
/// Volume in cubic feet
pub fn cups_cf(n: f64) -> f64 {
    n * CUPS_TO_CUBIC_FEET
}

/// Converts Cups to Gallons.
///
/// # Arguments
///
/// * `n` - Volume in cups
///
/// # Returns
///
/// Volume in gallons
pub fn cups_gal(n: f64) -> f64 {
    n * CUPS_TO_GALLONS
}

/// Converts Cubic Feet to Liters.
///
/// # Arguments
///
/// * `n` - Volume in cubic feet
///
/// # Returns
///
/// Volume in liters
pub fn cf_lit(n: f64) -> f64 {
    n * CUBIC_FEET_TO_LITERS
}

/// Converts Cubic Feet to Cubic Inches.
///
/// # Arguments
///
/// * `n` - Volume in cubic feet
///
/// # Returns
///
/// Volume in cubic inches
pub fn cf_ci(n: f64) -> f64 {
    n * CUBIC_FEET_TO_CUBIC_INCHES
}

/// Converts Cubic Feet to Tablespoons.
///
/// # Arguments
///
/// * `n` - Volume in cubic feet
///
/// # Returns
///
/// Volume in tablespoons
pub fn cf_tab(n: f64) -> f64 {
    n * CUBIC_FEET_TO_TABLESPOONS
}

/// Converts Cubic Feet to Cups.
///
/// # Arguments
///
/// * `n` - Volume in cubic feet
///
/// # Returns
///
/// Volume in cups
pub fn cf_cups(n: f64) -> f64 {
    n * CUBIC_FEET_TO_CUPS
}

/// Converts Cubic Feet to Gallons.
///
/// # Arguments
///
/// * `n` - Volume in cubic feet
///
/// # Returns
///
/// Volume in gallons
pub fn cf_gal(n: f64) -> f64 {
    n * CUBIC_FEET_TO_GALLONS
}

/// Converts Gallons to Liters.
///
/// # Arguments
///
/// * `n` - Volume in gallons
///
/// # Returns
///
/// Volume in liters
pub fn gal_lit(n: f64) -> f64 {
    n * GALLONS_TO_LITERS
}

/// Converts Gallons to Cubic Inches.
///
/// # Arguments
///
/// * `n` - Volume in gallons
///
/// # Returns
///
/// Volume in cubic inches
pub fn gal_ci(n: f64) -> f64 {
    n * GALLONS_TO_CUBIC_INCHES
}

/// Converts Gallons to Tablespoons.
///
/// # Arguments
///
/// * `n` - Volume in gallons
///
/// # Returns
///
/// Volume in tablespoons
pub fn gal_tab(n: f64) -> f64 {
    n * GALLONS_TO_TABLESPOONS
}

/// Converts Gallons to Cubic Feet.
///
/// # Arguments
///
/// * `n` - Volume in gallons
///
/// # Returns
///
/// Volume in cubic feet
pub fn gal_cf(n: f64) -> f64 {
    n * GALLONS_TO_CUBIC_FEET
}

/// Converts Gallons to Cups.
///
/// # Arguments
///
/// * `n` - Volume in gallons
///
/// # Returns
///
/// Volume in cups
pub fn gal_cups(n: f64) -> f64 {
    n * GALLONS_TO_CUPS
}


// Unit tests go here
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
        assert!(approx_eq!(f64, 9.0 / 11.0, 0.8182, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test Fahrenheit to Celsius functions
    fn test_fah_cel() {
        let f = 70.0;
        let c = 21.11;
        let res = fah_cel(f);
        assert!(approx_eq!(f64, res, c, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Fahrenheit to kelvin functions
    fn test_fah_kel() {
        let f = 70.0;
        let k = 294.26;
        let res = fah_kel(f);
        assert!(approx_eq!(f64, res, k, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Fahrenheit to Rankine functions
    fn test_fah_ran() {
        let f = 70.00;
        let r = 529.67;
        let res = fah_ran(f);
        assert!(approx_eq!(f64, res, r, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Celsius to Kelvin functions
    fn test_cel_kel() {
        let c = 70.0;
        let k = 343.15;
        let res = cel_kel(c);
        assert!(approx_eq!(f64, res, k, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Celsius to Fahrenheit functions
    fn test_cel_fah() {
        let c = 70.0;
        let f = 158.0;
        let res = cel_fah(c);
        assert!(approx_eq!(f64, res, f, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Celsius to Rankine functions
    fn test_cel_ran() {
        let c = 70.0;
        let r = 617.67;
        let res = cel_ran(c);
        assert!(approx_eq!(f64, res, r, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Kelvin to Celsius functions
    fn test_kel_cel() {
        let k = 70.0;
        let c = -203.15;
        let res = kel_cel(k);
        assert!(approx_eq!(f64, res, c, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Kelvin to Fahrenheit functions
    fn test_kel_fah() {
        let k = 70.0;
        let f = -333.67;
        let res = kel_fah(k);
        assert!(approx_eq!(f64, res, f, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }
    #[test]
    // test Kelvin to Rankine functions
    fn test_kel_ran() {
        let k = 70.0;
        let r = 126.0;
        let res = kel_ran(k);
        assert!(approx_eq!(f64, res, r, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    // Volume conversion tests
    // ----------------------------------------
    #[test]
    // test liters to tablespoons functions
    fn test_lit_tab() {
        let l = 1.0;
        let t = 67.628;
        let res = lit_tab(l);
        assert!(approx_eq!(f64, res, t, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test liters to cubic-inches functions
    fn test_lit_ci() {
        let l = 1.0;
        let ci = 61.023;
        let res = lit_ci(l);
        assert!(approx_eq!(f64, res, ci, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test liters to cups functions
    fn test_lit_cups() {
        let l = 1.0;
        let cups = 4.226;
        let res = lit_cups(l);
        assert!(approx_eq!(f64, res, cups, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test liters to cubic-feet functions
    fn test_lit_cf() {
        let l = 1.0;
        let cf = 0.0353;
        let res = lit_cf(l);
        assert!(approx_eq!(f64, res, cf, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test liters to gallons functions
    fn test_lit_gal() {
        let l = 1.0;
        let gal = 0.2641;
        let res = lit_gal(l);
        assert!(approx_eq!(f64, res, gal, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test tablespoons to liters functions
    fn test_tab_lit() {
        let t = 100.0;
        let l = 1.47;
        let res = tab_lit(t);
        assert!(approx_eq!(f64, res, l, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test tablespoons to cubic-inches functions
    fn test_tab_ci() {
        let t = 10.0;
        let ci = 9.02;
        let res = tab_ci(t);
        assert!(approx_eq!(f64, res, ci, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test tablespoons to cups functions
    fn test_tab_cups() {
        let t = 16.0;
        let cups = 0.992;
        let res = tab_cups(t);
        assert!(approx_eq!(f64, res, cups, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test tablespoons to cubic-feet functions
    fn test_tab_cf() {
        let t = 1000.0;
        let cf = 0.52219;
        let res = tab_cf(t);
        assert!(approx_eq!(f64, res, cf, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test tablespoons to gallons functions
    fn test_tab_gal() {
        let t = 256.0;
        let gal = 1.0;
        let res = tab_gal(t);
        assert!(approx_eq!(f64, res, gal, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-inches to liters functions
    fn test_ci_lit() {
        let ci = 100.0;
        let l = 1.63;
        let res = ci_lit(ci);
        assert!(approx_eq!(f64, res, l, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-inches to tablespoons functions
    fn test_ci_tab() {
        let ci = 10.0;
        let t = 11.0823;
        let res = ci_tab(ci);
        assert!(approx_eq!(f64, res, t, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-inches to cups functions
    fn test_ci_cups() {
        let ci = 100.0;
        let cups = 6.926;
        let res = ci_cups(ci);
        assert!(approx_eq!(f64, res, cups, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-inches to cubic-feet functions
    fn test_ci_cf() {
        let ci = 1728.0;
        let cf = 1.0;
        let res = ci_cf(ci);
        assert!(approx_eq!(f64, res, cf, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-inches to gallons functions
    fn test_ci_gal() {
        let ci = 231.0;
        let gal = 1.0;
        let res = ci_gal(ci);
        assert!(approx_eq!(f64, res, gal, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cups to liters functions
    fn test_cups_lit() {
        let cups = 4.0;
        let l = 0.946352;
        let res = cups_lit(cups);
        assert!(approx_eq!(f64, res, l, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cups to cubic-inches functions
    fn test_cups_ci() {
        let cups = 1.0;
        let ci = 14.4375;
        let res = cups_ci(cups);
        assert!(approx_eq!(f64, res, ci, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cups to tablespoons functions
    fn test_cups_tab() {
        let cups = 1.0;
        let t = 16.0;
        let res = cups_tab(cups);
        assert!(approx_eq!(f64, res, t, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cups to cubic-feet functions
    fn test_cups_cf() {
        let cups = 100.0;
        let cf = 0.835;
        let res = cups_cf(cups);
        assert!(approx_eq!(f64, res, cf, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cups to gallons functions
    fn test_cups_gal() {
        let cups = 16.0;
        let gal = 1.0;
        let res = cups_gal(cups);
        assert!(approx_eq!(f64, res, gal, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-feet to liters functions
    fn test_cf_lit() {
        let cf = 1.0;
        let l = 28.3168;
        let res = cf_lit(cf);
        assert!(approx_eq!(f64, res, l, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-feet to cubic-inches functions
    fn test_cf_ci() {
        let cf = 1.0;
        let ci = 1728.0;
        let res = cf_ci(cf);
        assert!(approx_eq!(f64, res, ci, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-feet to tablespoons functions
    fn test_cf_tab() {
        let cf = 1.0;
        let t = 1915.01;
        let res = cf_tab(cf);
        assert!(approx_eq!(f64, res, t, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-feet to cups functions
    fn test_cf_cups() {
        let cf = 1.0;
        let cups = 119.688;
        let res = cf_cups(cf);
        assert!(approx_eq!(f64, res, cups, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test cubic-feet to gallons functions
    fn test_cf_gal() {
        let cf = 1.0;
        let gal = 7.48052;
        let res = cf_gal(cf);
        assert!(approx_eq!(f64, res, gal, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test gallons to liters functions
    fn test_gal_lit() {
        let gal = 1.0;
        let l = 3.785;
        let res = gal_lit(gal);
        assert!(approx_eq!(f64, res, l, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test gallons to cubic-inches functions
    fn test_gal_ci() {
        let gal = 1.0;
        let ci = 231.0;
        let res = gal_ci(gal);
        assert!(approx_eq!(f64, res, ci, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test gallons to tablespoons functions
    fn test_gal_tab() {
        let gal = 1.0;
        let t = 256.0;
        let res = gal_tab(gal);
        assert!(approx_eq!(f64, res, t, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test gallons to cubic-feet functions
    fn test_gal_cf() {
        let gal = 1.0;
        let cf = 0.133;
        let res = gal_cf(gal);
        assert!(approx_eq!(f64, res, cf, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test gallons to cups functions
    fn test_gal_cups() {
        let gal = 1.0;
        let cups = 16.0;
        let res = gal_cups(gal);
        assert!(approx_eq!(f64, res, cups, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    // Round-trip conversion tests
    // ----------------------------------------
    #[test]
    // test round-trip: liters -> tablespoons -> liters
    fn test_roundtrip_lit_tab() {
        let original = 2.5;
        let converted = lit_tab(original);
        let back = tab_lit(converted);
        assert!(approx_eq!(f64, original, back, (ROUNDTRIP_TOLERANCE_EPSILON, ROUNDTRIP_TOLERANCE_ULPS)));
    }

    #[test]
    // test round-trip: gallons -> cubic-feet -> gallons
    fn test_roundtrip_gal_cf() {
        let original = 5.0;
        let converted = gal_cf(original);
        let back = cf_gal(converted);
        assert!(approx_eq!(f64, original, back, (ROUNDTRIP_TOLERANCE_EPSILON, ROUNDTRIP_TOLERANCE_ULPS)));
    }

    #[test]
    // test round-trip: cups -> cubic-inches -> cups
    fn test_roundtrip_cups_ci() {
        let original = 3.0;
        let converted = cups_ci(original);
        let back = ci_cups(converted);
        assert!(approx_eq!(f64, original, back, (ROUNDTRIP_TOLERANCE_EPSILON, ROUNDTRIP_TOLERANCE_ULPS)));
    }

    // Enum-based conversion tests
    // ----------------------------------------
    #[test]
    // test enum-based conversion: Celsius to Kelvin
    fn test_enum_convert_celsius_kelvin() {
        let result = convert(Unit::Celsius, Unit::Kelvin, 70.0);
        assert!(result.is_some());
        assert!(approx_eq!(f64, result.unwrap(), 343.15, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test enum-based conversion: Fahrenheit to Celsius
    fn test_enum_convert_fahrenheit_celsius() {
        let result = convert(Unit::Fahrenheit, Unit::Celsius, 70.0);
        assert!(result.is_some());
        assert!(approx_eq!(f64, result.unwrap(), 21.11, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test enum-based conversion: Liters to Gallons
    fn test_enum_convert_liters_gallons() {
        let result = convert(Unit::Liters, Unit::Gallons, 1.0);
        assert!(result.is_some());
        assert!(approx_eq!(f64, result.unwrap(), 0.2641, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test enum-based conversion: Cups to Tablespoons
    fn test_enum_convert_cups_tablespoons() {
        let result = convert(Unit::Cups, Unit::Tablespoons, 1.0);
        assert!(result.is_some());
        assert!(approx_eq!(f64, result.unwrap(), 16.0, (TOLERANCE_EPSILON, TOLERANCE_ULPS)));
    }

    #[test]
    // test enum-based conversion: same unit returns same value
    fn test_enum_convert_same_unit() {
        let result = convert(Unit::Celsius, Unit::Celsius, 100.0);
        assert_eq!(result, Some(100.0));
    }

    #[test]
    // test enum-based conversion: invalid conversion (temperature to volume)
    fn test_enum_convert_invalid() {
        let result = convert(Unit::Celsius, Unit::Liters, 100.0);
        assert!(result.is_none());
    }

    #[test]
    // test Unit::from_str parsing
    fn test_unit_from_str() {
        assert_eq!(Unit::from_str("celsius"), Ok(Unit::Celsius));
        assert_eq!(Unit::from_str("CELSIUS"), Ok(Unit::Celsius));
        assert_eq!(Unit::from_str("kelvin"), Ok(Unit::Kelvin));
        assert_eq!(Unit::from_str("liters"), Ok(Unit::Liters));
        assert_eq!(Unit::from_str("cubic-inches"), Ok(Unit::CubicInches));
        assert_eq!(Unit::from_str("cubic-feet"), Ok(Unit::CubicFeet));
        assert_eq!(
            Unit::from_str("invalid"),
            Err(UnitParseError::UnknownUnit("invalid".to_string()))
        );
    }
}
