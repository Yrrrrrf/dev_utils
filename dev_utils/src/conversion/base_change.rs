//! Convert numbers from one base to another.
//! 
//! This module contains functions for converting numbers from one base to another.
//!
//! The input string must have the form `[A-Za-z\d]+(\.[A-Za-z\d]+)?`, where:
//!
//! - `[A-Za-z\d]+` represents any number of letters or digits (at least one).
//! - `(\.[A-Za-z\d]+)?` represents an optional decimal part, which consists of a dot followed by any number of letters or digits (at least one).
//! 
//! The function supports bases up to 62, which means that the input string can contain any of the following characters: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`.
//! 
//! # Examples
//! ```rust
//! use dev_utils::conversion::base_change::*;
//! 
//! let result = str_to_num_from_base("1010", 2, 16);
//! assert_eq!(result, Ok("A".to_string()));
//! 
//! let result = str_to_num_from_base("1010.1010", 2, 16);
//! assert_eq!(result, Ok("A.A".to_string()));
//! ```


/// The function supports bases up to 62, which means that the input string can contain any of the following characters: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`.
///
/// # Arguments
///
/// - `src` - A string to convert to a number.
/// - `src_base` - The base of the source number.
/// - `new_base` - The base of the new number.
///
/// # Returns
///
/// A string that represents the number in the new base, or an error message if the input is invalid.
///
/// # Examples
/// ```
/// use dev_utils::conversion::base_change::str_to_num_from_base;
///
/// let result = str_to_num_from_base("1010", 2, 16);
/// assert_eq!(result, Ok("A".to_string()));
/// ```
///
/// # Errors
///
/// The function returns an error message if any of the following conditions are met:
///
/// - The input bases are not between 2 and 62.
/// - The input string contains an invalid character for the given base.
///
/// # Implementation details
///
/// The function first validates the input bases and characters, and then converts the input string to its base 10 equivalent. It then converts the integer part of the number to the new base using the `int_b10_dst` function, and adds the decimal part of the number to the new number if it exists. Finally, it returns the new number as a string.
pub fn str_to_num_from_base(src: &str, src_base: u8, new_base: u8) -> Result<String, String> {
    // src.chars().for_each(|c| println!("{} -> {:?}", c, digit_value(c)));  // Print the value of all the chars in the string
    //* Validate the bases
    // The maximum base is 62 because the alphabet is limited to 62 characters
    if src_base < 2 || src_base > 62 || new_base < 2 || new_base > 62 {
        return Err("Invalid base (must be between 2 & 62)".to_string())
    }
    //* Validate that all the characters in the string are valid for the given base
    if src.chars().any(|c| digit_value(c).unwrap_or(0) >= src_base) {
        return Err("Invalid character for the given base".to_string())
    }
    let mut src = src.chars();  // Create an iterator for the string
    //* Get base 10 equivalent of the number
    let b10_int = src_int_b10(&src.by_ref().take_while(|c| c != &'.' && c != &',').collect::<String>(), src_base);
    let b10_dec = src_dec_b10(&src.by_ref().collect::<String>(), src_base);
    //* Convert the integer part of the number to the new base
    let mut new_number = int_b10_dst(b10_int, new_base);

    if b10_dec > 0.00001 {  // If the decimal part of the number is greater than 0
        new_number.push('.');  // Add a dot to the new number
        new_number.push_str(&dec_b10_dst(b10_dec, new_base));  // Add the decimal part of the number to the new number
        // remove the last 000000's from the number
        new_number = new_number.chars().rev().skip_while(|c| c == &'0').collect::<String>().chars().rev().collect::<String>();
    }
    // remove the last 000000's from the number
    Ok(new_number)  // Return the new number
}


/// Converts a character to its numerical value in the given base.
///
/// The function can interpret any character in the alphabet as a digit, where:
///
/// - `0..9` ->  0.. 9
/// - `A..Z` -> 10..35
/// - `a..z` -> 36..61
///
/// # Arguments
///
/// - `c` - A character to convert to a number.
///
/// # Returns
///
/// An [`Option<u8>`] that represents the numerical value of the character, or `None` if the character is not a valid digit.
///
/// # Examples
/// ```
/// use dev_utils::conversion::base_change::digit_value;
///
/// assert_eq!(digit_value('A'), Some(10));
/// assert_eq!(digit_value('E'), Some(14));
/// assert_eq!(digit_value('Z'), Some(35));
/// 
/// ```
pub fn digit_value(c: char) -> Option<u8> {  // Function to convert a digit character to its numerical value
    match c {  // Match the character
        '0'..='9' => Some(c as u8 - '0' as u8),
        'A'..='Z' => Some(c as u8 - 'A' as u8 + 10),
        'a'..='z' => Some(c as u8 - 'a' as u8 + 36),
        _ => None  // If c is not a valid character, return an error
    }
}


/// Converts the integer part of a number from the given source base to base 10.
///
/// # Arguments
///
/// - `src` - A string slice that represents the integer part of the number to convert.
/// - `src_base` - An unsigned 8-bit integer that represents the source base of the number to convert.
///
/// # Return
///
/// An unsigned 32-bit integer that represents the integer part of the number in base 10.
///
/// # Panics
///
/// This function will panic if the input string contains an invalid character for the given source base.
///
/// # Examples
/// ```
/// use dev_utils::conversion::base_change::src_int_b10;
///
/// assert_eq!(src_int_b10("1010", 2), 10);
/// ```
pub fn src_int_b10(src: &str, src_base: u8) -> u32 {
    let mut n = 0;  // Create a variable to store the integer part of the number
    for (i, c) in src.chars().rev().enumerate() {
        let digit = digit_value(c).unwrap_or_else(|| panic!("Invalid character: {}", c));
        let digit = digit as u32 * (src_base as u32).pow(i as u32);
        n += digit;
    }
    n
}


/// Converts an integer from base 10 to the given destination base and returns the result as a string.
///
/// # Arguments
///
/// - `n` - An unsigned 32-bit integer that represents the integer to convert.
/// - `dst_base` - An unsigned 8-bit integer that represents the destination base of the conversion.
///
/// # Returns
///
/// A string that represents the integer in the destination base.
///
/// # Examples
/// ```
/// use dev_utils::conversion::base_change::int_b10_dst;
///
/// assert_eq!(int_b10_dst(42, 16), "2A");
/// ```
pub fn int_b10_dst(n: u32, dst_base: u8) -> String {
    let mut n = n;  // Create a variable to store the integer part of the number
    let mut new_number = String::new();  // Create a string to store the new number
    while n > 0 {  // While the number is greater than 0
        let digit = n % dst_base as u32;  // Get the remainder of the division
        let digit = if digit < 10 {digit as u8 + '0' as u8} else {digit as u8 - 10 + 'A' as u8};  // Convert the digit to a char
        new_number.insert(0, digit as char);  // Insert the digit at the beginning of the string
        n /= dst_base as u32;  // Divide the number by the base
    }
    new_number  // Return the new number in the new base as a string
}


// & DECIMAL PARTS


/// Converts the decimal part of a number from the given source base to base 10.
///
/// # Arguments
///
/// - `src` - A string slice that represents the decimal part of the number to convert.
/// - `src_base` - An unsigned 8-bit integer that represents the source base of the number to convert.
///
/// # Returns
///
/// A float that represents the decimal part of the number in base 10.
///
/// # Panics
///
/// This function will panic if the input string contains an invalid character for the given source base.
///
/// # Examples
/// ```
/// use dev_utils::conversion::base_change::src_dec_b10;
///
/// assert_eq!(src_dec_b10("101", 2), 0.625);
/// ```
pub fn src_dec_b10(src: &str, src_base: u8) -> f64 {
    let mut d = 0.0;  // Create a variable to store the decimal part of the number
    for (i, c) in src.chars().enumerate() {
        let digit = digit_value(c).unwrap_or_else(|| panic!("Invalid character: {}", c));
        let digit = digit as f64 * (src_base as f64).powi(-(i as i32 + 1));  // Calculate the value of the digit in base 10
        d += digit;  // Add the digit value to the decimal part of the number
    }
    d  // Return the decimal part of the number in base 10 as a float
}


/// Converts a decimal number from base 10 to the given destination base and returns the result as a string.
///
/// `Note`: This function is not very precise.
/// It can only handle 8 exact decimals. Then it starts to lose precision.
/// 
/// # Arguments
///
/// - `d` - A float that represents the decimal number to convert.
/// - `dst_base` - An unsigned 8-bit integer that represents the destination base of the conversion.
///
/// # Returns
///
/// A string that represents the decimal number in the destination base.
///
/// # Examples
/// ```
/// use dev_utils::conversion::base_change::dec_b10_dst;
///
/// assert_eq!(dec_b10_dst(0.625, 2), "101");
/// ```
pub fn dec_b10_dst(mut d: f64, dst_base: u8) -> String {
    let mut new_number = String::new();  // Create a string to store the new number

    for _ in 0..8 {  // While the number is greater than 0
        d *= dst_base as f64;  // Multiply the number by the base
        let digit = d as u8;  // Get the integer part of the number
        let digit = if digit < 10 {digit as u8 + '0' as u8} else {digit as u8 - 10 + 'A' as u8};  // Convert the digit to a char
        new_number.push(digit as char);  // Insert the digit at the beginning of the string
        d -= digit as f64;  // Subtract the integer part of the number from the number
    }
    new_number
}
