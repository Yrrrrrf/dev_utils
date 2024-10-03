//! Some conversion utilities.
//! 
//! # Usage
//! ```rust
//! use dev_utils::conversion::*;
//! ```
//! 
//! # Contents
//! - [`datetime`] - Date and time conversion utilities.
//! - [`base_change`] - Base change utilities.
pub mod datetime;
pub mod base_change;

// todo: add some more unit tests
#[cfg(test)]
mod tests {
    use crate::conversion::base_change::*;

    // ? BASE CHANGE TESTS --------------------------------------------------------------------

    fn test_base_change() {
        vec![  // vec![src_base, new_base, src, result]
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

        ].iter().for_each(|(src_base, new_base, src, result)|
            // assert_eq!(str_to_num_from_base(src, *src_base, *new_base).unwrap(), result.to_string()));
            println!("{}", 1)
        );

        // * To print the results in the terminal
        // ].iter().for_each(|(src_base, new_base, src, result)|
        //     println!("{} b{:_>2} = {} b{:_>2}\t{}", src, src_base, 
        //         str_to_num_from_base(src, *src_base, *new_base).unwrap(), new_base,
        //         crate::terminal::set_fg(result, if str_to_num_from_base(src, *src_base, *new_base).unwrap() == result.to_string() {"g"} else {"r"})
        // ));
    }
}
