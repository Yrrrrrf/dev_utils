#![allow(unused)]

use std::{thread, time::Duration};

use dev_utils::{
    app_dt, conversion::base_change::*, debug, dlog::*, error, format::*, info, trace, warn
};

fn main() {
    app_dt!(file!());
    // app_dt!(file!(), "package" => ["license", "keywords"]);


    // test_base_conversion();
}


fn test_base_conversion() {
    println!("\n--- Base Conversion Tests ---");

    // Test decimal to binary
    let result = convert_base("10.5", 10, 2).unwrap();
    println!("10.5 (base 10) to base 2: {}", result);
    assert_eq!(result, "1010.1");

    // Test binary to decimal
    let result = convert_base("1010.1", 2, 10).unwrap();
    println!("1010.1 (base 2) to base 10: {}", result);
    assert_eq!(result, "10.5");

    // Test with integer
    let num = 42;
    let base = 10;
    let new_base = 16;
    let num_str = num.to_string();
    let result = convert_base(&num_str, base, new_base).unwrap();
    println!("{} in base {} is {} in base {}", num, base, result, new_base);

    // Convert back
    let result_back = convert_base(&result, new_base, base).unwrap();
    println!("{} in base {} is {} in base {}", result, new_base, result_back, base);
    assert_eq!(num_str, result_back);
}

fn __delay(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("\t----Delay---- ({ms} ms)");
}
