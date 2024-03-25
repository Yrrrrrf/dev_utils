#![allow(dead_code)]  // Allow dead code in a file or globally


use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)



use device_query::{DeviceQuery, DeviceState, MouseState, Keycode, device_state};  // device_query library


/// Runs the module
pub fn run() {
    println!("Module path: {}\n\n", module_path!());


    // print_event();


    // ~ Log time ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    println!("Time now: {}", chrono::Utc::now());  // UTC time
    let time_mexico = chrono::Utc::now() + chrono::Duration::hours(-6);  // Mexico time
    println!("Mexico:  {:>32}", time_mexico.time().to_string());  // Mexico time
    let time_shanghai = chrono::Utc::now() + chrono::Duration::hours(8);  // Shanghai time
    println!("Shanghai:{:>32}", time_shanghai.time().to_string());  // Shanghai time


    // ? TEST -------------------------------------------------------------------------------------------









    // count the execution time of a matrix multiplication
    let mut matrix_a: Vec<Vec<i32>> = Vec::new();
    let mut matrix_b: Vec<Vec<i32>> = Vec::new();

    // matrix 
    let matrix_size: usize = 10;

    // create the matrixes
    for i in 0..matrix_size {
        matrix_a.push(Vec::new());
        matrix_b.push(Vec::new());
        for j in 0..matrix_size {
            matrix_a[i].push(i as i32 + j as i32);
            matrix_b[i].push(i as i32 - j as i32);
        }
    }

    // multiply the matrixes
    let mut matrix_c: Vec<Vec<i32>> = Vec::new();
    let mut time_old = chrono::Utc::now().timestamp_nanos();

    // iterate over the rows of the matrix
    for i in 0..matrix_a.len() {
        matrix_c.push(Vec::new());  // create a new row
        for j in 0..matrix_b[0].len() {
            matrix_c[i].push(0);
            for k in 0..matrix_b.len() {
                matrix_c[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }

    let time_new = chrono::Utc::now().timestamp_nanos();

    println!("Time: {} ns", time_new - time_old);
    println!("Matrix C: {:?}", matrix_c[0][0]);





}


/// Prints always a key is pressed or unpressed
pub fn print_event() {
    let mut old_keys: Vec<Keycode> = Vec::new();  // create a new vector of Keycode (empty)

    loop {
        // ? Keyboard events
        let new_keys: Vec<Keycode> = DeviceState::new().get_keys(); // get the current pressed keys
        if new_keys.contains(&Keycode::Escape) {break;}  // if escape is pressed, then break the loop

        new_keys.iter().for_each(|key| if !old_keys.contains(key) { println!("↓ {:?}", key); });  // print when a key is   PRESSED
        old_keys.iter().for_each(|key| if !new_keys.contains(key) { println!("↑ {:?}", key); });  // print when a key is UNPRESSED

        old_keys = new_keys.clone();  // update the old keys
    }
}

