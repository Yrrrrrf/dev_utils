//! Base conversion module for arbitrary-precision numbers.
//!
//! This module provides functionality to convert numbers between different bases,
//! supporting bases from 2 to 62. It handles both integer and fractional numbers,
//! and uses a custom [BigUint] implementation for arbitrary-precision arithmetic.
//!
//! # Features
//! - Convert numbers between any base from 2 to 62
//! - Support for fractional numbers
//! - Arbitrary-precision arithmetic using [BigUint]
//!
//! # Examples
//! ```
//! use dev_utils::base_change::convert_base;
//!
//! assert_eq!(convert_base("1010", 2, 10).unwrap(), "10");
//! assert_eq!(convert_base("FF", 16, 10).unwrap(), "255");
//! ```
use std::fmt;

/// A custom arbitrary-precision unsigned integer implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUint {
    pub digits: Vec<u8>, // any number N base u8 (2^8 = 256 -> 0..=255)
}

impl BigUint {
    /// Creates a new `BigUint` initialized to zero.
    ///
    /// # Returns
    /// A new `BigUint` instance with a single digit of value 0.
    fn new() -> Self {
        BigUint { digits: vec![0] }
    }

    /// Creates a new `BigUint` from a u8 value.
    ///
    /// # Arguments
    /// * `n` - The u8 value to convert to a `BigUint`.
    ///
    /// # Returns
    /// A new `BigUint` instance representing the given u8 value.
    fn from_u8(n: u8) -> Self {
        BigUint { digits: vec![n] }
    }

    /// Checks if the `BigUint` is zero.
    ///
    /// # Returns
    /// `true` if the `BigUint` is zero, `false` otherwise.
    fn is_zero(&self) -> bool {
        self.digits.iter().all(|&d| d == 0)
    }
    /// Multiplies the `BigUint` by a small (u8) number.
    ///
    /// # Arguments
    /// * `n` - The u8 value to multiply by.
    ///
    /// This method modifies the `BigUint` in place.
    fn mul_small(&mut self, n: u8) {
        let mut carry = 0;
        for d in &mut self.digits {
            let prod = *d as u16 * n as u16 + carry;
            *d = (prod % 256) as u8;
            carry = prod / 256;
        }
        if carry > 0 {
            self.digits.push(carry as u8)
        }
    }

    /// Adds a small (u8) number to the `BigUint`.
    ///
    /// # Arguments
    /// * `n` - The u8 value to add.
    ///
    /// This method modifies the `BigUint` in place.
    fn add_small(&mut self, n: u8) {
        let mut carry = n;
        for d in &mut self.digits {
            let sum = *d as u16 + carry as u16;
            *d = (sum % 256) as u8;
            carry = (sum / 256) as u8;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }

    /// Divides the `BigUint` by a small (u16) number and returns the remainder.
    ///
    /// # Arguments
    /// * `n` - The u16 value to divide by.
    ///
    /// # Returns
    /// The remainder of the division as a u8.
    ///
    /// This method modifies the `BigUint` in place, storing the quotient.
    fn div_mod_small(&mut self, n: u16) -> u8 {
        let mut remainder = 0u16;
        for d in self.digits.iter_mut().rev() {
            let dividend = remainder * 256 + *d as u16;
            *d = (dividend / n) as u8;
            remainder = dividend % n;
        }
        while self.digits.len() > 1 && self.digits.last() == Some(&0) {
            self.digits.pop();
        }
        remainder as u8
    }
}

/// A fixed-point decimal number representation.
#[derive(Debug, Clone, PartialEq)]
pub struct FixedDecimal {
    pub value: BigUint,
    pub scale: u32,
}

impl FixedDecimal {
    /// Creates a new [FixedDecimal] from a [BigUint] value and a scale.
    ///
    /// # Arguments
    /// * `value` - The `BigUint` value.
    /// * `scale` - The number of decimal places.
    fn new(value: BigUint, scale: u32) -> Self {
        FixedDecimal { value, scale }
    }

    /// Parses a string representation of a number in a given base into a [FixedDecimal].
    ///
    /// # Arguments
    /// * `s` - The string to parse.
    /// * `radix` - The base of the number system (2-62).
    ///
    /// # Returns
    /// A [Result] containing either the parsed [FixedDecimal] or a [BaseConversionError].
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, BaseConversionError> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 2 {
            return Err(BaseConversionError::InvalidInput);
        }

        let mut value = BigUint::new();
        let mut scale = 0;

        // Integer part
        for &c in parts[0].as_bytes() {
            let digit = digit_to_val(c)?;
            if digit >= radix as u8 {
                return Err(BaseConversionError::InvalidDigit);
            }
            value.mul_small(radix as u8);
            value.add_small(digit);
        }

        // Fractional part
        if parts.len() == 2 {
            for &c in parts[1].as_bytes() {
                let digit = digit_to_val(c)?;
                if digit >= radix as u8 {
                    return Err(BaseConversionError::InvalidDigit);
                }
                value.mul_small(radix as u8);
                value.add_small(digit);
                scale += 1;
            }
        }

        Ok(FixedDecimal { value, scale })
    }

    /// Converts the `FixedDecimal` to a string representation in the specified base.
    ///
    /// # Arguments
    /// * `radix` - The base to convert to (2-62).
    ///
    /// # Returns
    /// A [String] representing the number in the specified base.
    fn to_string_radix(&self, radix: u32) -> String {
        if self.value.is_zero() {
            return "0".to_string();
        }

        let mut int_part = self.value.clone();
        let mut frac_part = BigUint::new();

        for _ in 0..self.scale {
            let remainder = int_part.div_mod_small(radix as u16);
            frac_part.mul_small(radix as u8);
            frac_part.add_small(remainder);
        }

        let mut result = int_part_to_string_radix(&int_part, radix);

        if !frac_part.is_zero() {
            result.push('.');
            for _ in 0..self.scale {
                frac_part.mul_small(radix as u8);
                let digit = frac_part.div_mod_small(256);
                result.push(val_to_digit(digit));
            }
        }

        result
    }
}

/// Represents errors that can occur during base conversion.
#[derive(Debug)]
pub enum BaseConversionError {
    InvalidBase,
    InvalidDigit,
    InvalidInput,
}

/// Converts a digit character to its numeric value.
///
/// # Arguments
/// * `c` - The character to convert.
///
/// # Returns
/// A `Result` containing either the numeric value or a [BaseConversionError].
pub fn digit_to_val(c: u8) -> Result<u8, BaseConversionError> {
    match c {
        // todo: Improve this macro to now be able to:
        // todo: - handle more than 62 bases (the problem is how the define some custom ALPHABET)
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'Z' => Ok(c - b'A' + 10),
        b'a'..=b'z' => Ok(c - b'a' + 36),
        _ => Err(BaseConversionError::InvalidDigit),
    }
}

/// Converts a numeric value to its digit character representation.
///
/// # Arguments
/// * `v` - The numeric value to convert.
///
/// # Returns
/// The character representation of the digit.
pub fn val_to_digit(v: u8) -> char {
    match v {
        0..=9 => (v + b'0') as char,
        10..=35 => (v - 10 + b'A') as char,
        36..=61 => (v - 36 + b'a') as char,
        _ => unreachable!(),
    }
}

/// Converts the integer part of a [BigUint] to a string in the specified base.
///
/// # Arguments
/// * `n` - The `BigUint` to convert.
/// * `radix` - The base to convert to.
///
/// # Returns
/// A `String` representing the integer part in the specified base.
fn int_part_to_string_radix(n: &BigUint, radix: u32) -> String {
    if n.is_zero() {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut n = n.clone();
    while !n.is_zero() {
        let digit = n.div_mod_small(radix as u16);
        result.insert(0, val_to_digit(digit));
    }
    result
}

/// Converts a number from one base to another.
///
/// # Arguments
/// * `number` - The number to convert, as a string.
/// * `from_base` - The base of the input number (2-62).
/// * `to_base` - The base to convert to (2-62).
///
/// # Returns
/// A `Result` containing either the converted number as a [String] or a [BaseConversionError].
///
/// # Examples
/// ```
/// use crate::base_change::convert_base;
///
/// assert_eq!(convert_base("1010", 2, 10).unwrap(), "10");
/// assert_eq!(convert_base("FF", 16, 10).unwrap(), "255");
/// ```
pub fn convert_base(
    number: &str,
    from_base: u32,
    to_base: u32,
) -> Result<String, BaseConversionError> {
    if !(2..=62).contains(&from_base) || !(2..=62).contains(&to_base) {
        return Err(BaseConversionError::InvalidBase);
    }

    let value = FixedDecimal::from_str_radix(number, from_base)?;
    Ok(value.to_string_radix(to_base))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{Style, Stylize};

    #[test]
    fn test_fractional_conversion() {
        assert_eq!(convert_base("0.5", 10, 2).unwrap(), "0.1");
        assert_eq!(
            convert_base("0.1", 10, 2).unwrap(),
            "0.0001100110011001100110011001100110011001100110011001101"
        );
        assert_eq!(convert_base("0.1", 2, 10).unwrap(), "0.5");
    }

    #[test]
    fn test_mixed_number_conversion() {
        // ^ The error here is related to the floating point precision...
        assert_eq!(convert_base("10.5", 10, 2).unwrap(), "1010.1");
        assert_eq!(convert_base("1010.1", 2, 10).unwrap(), "10.5");
    }

    #[test]
    fn test_large_number_conversion() {
        assert_eq!(convert_base("1000000", 10, 16).unwrap(), "F4240");
        assert_eq!(convert_base("F4240", 16, 10).unwrap(), "1000000");
    }

    #[test]
    fn test_base_62_conversion() {
        assert_eq!(
            convert_base("HelloWorld", 62, 10).unwrap(),
            "239032307299047885"
        );
        assert_eq!(
            convert_base("239032307299047885", 10, 62).unwrap(),
            "HelloWorld"
        );
    }

    #[test]
    fn test_zero_conversion() {
        assert_eq!(convert_base("0", 2, 10).unwrap(), "0");
        assert_eq!(convert_base("0", 10, 16).unwrap(), "0");
        assert_eq!(convert_base("0.0", 2, 10).unwrap(), "0");
    }

    #[test]
    fn test_error_handling() {
        // Invalid base
        assert!(convert_base("10", 1, 10).is_err());
        assert!(convert_base("10", 10, 63).is_err());

        // Invalid digit for given base
        assert!(convert_base("2", 2, 10).is_err());
        assert!(convert_base("G", 16, 10).is_err());

        // Invalid input format
        assert!(convert_base("1.2.3", 10, 2).is_err());
    }

    #[test]
    fn test_identity_conversion() {
        let number: &str = "1234567890";
        (2..=62).for_each(|base| {
            let converted = convert_base(number, 10, base).unwrap();
            let back_to_decimal = convert_base(&converted, base, 10).unwrap();
            assert_eq!(back_to_decimal, number);
        });
    }

    fn compare_precision(a: &str, b: &str, digits: usize) -> bool {
        let a_parts: Vec<&str> = a.split('.').collect();
        let b_parts: Vec<&str> = b.split('.').collect();

        // Compare integer parts
        if a_parts[0] != b_parts[0] {
            return false;
        }

        // If either number doesn't have a fractional part, they're only equal if digits == 0
        if a_parts.len() == 1 || b_parts.len() == 1 {
            return digits == 0;
        }

        let a_frac = a_parts[1].chars().take(digits);
        let b_frac = b_parts[1].chars().take(digits);

        a_frac.eq(b_frac)
    }

    #[test]
    fn test_precision_retention() {
        // ^ The error here is related to the floating point precision...
        let test_cases = [
            ("0.12345678901234567890", 10),
            ("0.1", 15),
            ("1.414213562373095", 8),
            ("3.141592653589793", 6),
        ]
        .iter()
        .for_each(|(original, precision)| {
            let binary = convert_base(original, 10, 2).unwrap();
            let back_to_decimal = convert_base(&binary, 2, 10).unwrap();

            assert!(
                compare_precision(original, &back_to_decimal, *precision),
                "Failed for {} with precision {}: got {}",
                original,
                precision,
                back_to_decimal
            );
        });
    }

    #[test]
    fn test_high_precision_conversion() {
        // ^ The error here is related to the floating point precision...
        let original = "0.1234567890123456789";
        let hex = convert_base(original, 10, 16).unwrap();
        let back_to_decimal = convert_base(&hex, 16, 10).unwrap();

        assert!(
            compare_precision(original, &back_to_decimal, 15),
            "High precision test failed. Original: {}, Result: {}",
            original,
            back_to_decimal
        );
    }

    #[test]
    fn test_precision_edge_cases() {
        assert!(compare_precision("0.5", "0.5", 1));
        assert!(compare_precision("0.5", "0.50", 2));
        assert!(!compare_precision("0.5", "0.51", 2));
        assert!(compare_precision("1.000", "1", 0));
        assert!(!compare_precision("1.000", "1", 1));
    }

    #[test]
    fn test_base_conversion() {
        vec![
            // vec![src_base, new_base, src, result]
            // bin -> dec
            (2, 10, "11011100", "220"),
            (2, 10, "110011", "51"),
            (2, 10, "11001100", "204"),
            (2, 10, "11110011", "243"),
            (2, 10, "1100111", "103"),
            // dec -> bin
            (10, 2, "197", "11000101"),
            (10, 2, "253", "11111101"),
            (10, 2, "79", "1001111"),
            (10, 2, "297", "100101001"),
            (10, 2, "528", "1000010000"),
            // bin -> hex
            (2, 16, "100111011", "13B"),
            (2, 16, "11011011", "DB"),
            (2, 16, "101111011", "17B"),
            (2, 16, "11011001", "D9"),
            (2, 16, "111011101", "1DD"),
            // hex -> bin
            (16, 2, "9F", "10011111"),
            (16, 2, "9BAF", "1001101110101111"),
            (16, 2, "8BCD", "1000101111001101"),
            (16, 2, "72BA", "111001010111010"),
            (16, 2, "987", "100110000111"),
            (16, 2, "9F27", "1001111100100111"),
            // bin -> oct
            (2, 8, "11011001", "331"),
            (2, 8, "100111001", "471"),
            (2, 8, "11100110", "346"),
            (2, 8, "11001100", "314"),
            (2, 8, "1101110", "156"),
            // oct -> bin
            (8, 2, "245", "10100101"),
            (8, 2, "327", "11010111"),
            (8, 2, "651", "110101001"),
            // ? Decimal numbers test
            // These aproximate numbers are not exact because of the floating point precision
            // So the result is not exact, but it's close enough
            // The str_to_num_from_base() fn returns the last number that is not 0. So the result is not exact
            // &Example: 0.102000 -> 0.102 (the last 0s are not returned)
            // TODO: FIX THE DECIMAL PART FUNCTIONS TO COMPARE THIS KIND OF NUMBERS
            // (10, 2, "450.5", "111000010.1"),
            // (10, 2, "8.5", "1000.1"),
            // (10, 8, "450.5", "702.4"),
            // (10, 8, "7.5", "7.4"),
            // (10, 16, "450.5", "1C2.8"),
            // (10, 16, "8.5", "8.8"),
            // (8, 10, "450.5", "296.625"),
            // (8, 10, "7.5", "7.625"),
            // (2, 10, "1010.1", "10.5"),
            // (20, 6, "AA.21", "550.034050123501235"),
            // (10, 16, "2197.42", "895.6B851EB851EB851"),
            // (16, 10, "9E.D", "158.8125"),
        ]
        .iter()
        .for_each(|(src_base, new_base, src, result)| {
            assert_eq!(convert_base(src, *src_base, *new_base).unwrap(), *result)
        });
    }
}
