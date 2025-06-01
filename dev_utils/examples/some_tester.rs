#![allow(unused)]

use std::{thread, time::Duration};

use dev_utils::datetime::{Date, DateTime, Time};
use dev_utils::dlog::set_max_level;
use dev_utils::format::{Color, Style, Stylize};
use dev_utils::{app_dt, read_input};
use std::str::FromStr;
fn main() {
    app_dt!(file!());
    // app_dt!(file!(), "package" => ["license", "keywords"]);

    set_max_level(dev_utils::dlog::Level::Trace);

    // test_io();
    // test_pause();
    // f();
    some();
}

fn test_io() {
    let age = read_input::<u32>(Some("Enter your age: ")).unwrap();
    let name = read_input::<String>(Some("Enter your name: ")).unwrap();
    println!("Hello, {}! You are {} years old.", name, age);
}

fn test_pause() {
    read_input::<i128>(Some("Press Enter to continue...")); // * Input prompt
    read_input::<String>(None); // * Silent pause
}

fn f() {
    let date = Date::new(2023, 5, 1).unwrap();
    let time = Time::new(12, 34, 56).unwrap();
    let dt = DateTime { date, time };
    assert_eq!(dt.to_string(), "2023-05-01 12:34:56");
    let parsed_dt = DateTime::from_str("2023-05-01 12:34:56").unwrap();
    assert_eq!(parsed_dt, dt);
}

fn some() {
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
        // dev_utils::info!("{} -> {} (base {})", src, result, new_base);
        dev_utils::info!(
            "{:>20} {} -> {:>24} {}",
            src.style(Style::Bold),
            format!("(b_{:<02})", src_base.to_string()).style(Style::Dim),
            result.style(Style::Bold),
            format!("(b_{:<02})", new_base.to_string()).style(Style::Dim)
        );

        assert_eq!(
            dev_utils::base_change::convert_base(src, *src_base, *new_base).unwrap(),
            *result
        )
    });
}
