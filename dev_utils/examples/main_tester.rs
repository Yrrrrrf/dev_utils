#![allow(unused)]

use std::{thread, time::Duration};

use dev_utils::{app_dt, read_input};
use dev_utils::datetime::{Date, Time, DateTime};
use std::str::FromStr;
fn main() {
    app_dt!(file!());
    // app_dt!(file!(), "package" => ["license", "keywords"]);

    test_io();
    test_pause();
    f();
}


fn test_io() {
    let age = read_input::<u32>(Some("Enter your age: ")).unwrap();
    let name = read_input::<String>(Some("Enter your name: ")).unwrap();
    println!("Hello, {}! You are {} years old.", name, age);    
}

fn test_pause() {
    read_input::<i128>(Some("Press Enter to continue..."));  // * Input prompt
    read_input::<String>(None);  // * Silent pause
}

fn f() {
    let date = Date::new(2023, 5, 1).unwrap();
    let time = Time::new(12, 34, 56).unwrap();
    let dt = DateTime { date, time };
    assert_eq!(dt.to_string(), "2023-05-01 12:34:56");
    let parsed_dt = DateTime::from_str("2023-05-01 12:34:56").unwrap();
    assert_eq!(parsed_dt, dt);

}