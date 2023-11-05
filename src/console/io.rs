use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


/// This ask() function is still a prototype, so **it could not work as expected**.
/// 
/// Ask for input from the console
/// 
/// ### Parameters:
/// - `T: std::str::FromStr` - The type of the input
/// 
/// ### Returns:
/// - `T` - The input
pub fn ask<T: std::str::FromStr>() -> T where <T as FromStr>::Err: std::fmt::Debug {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<T>().unwrap();
}


/// Print the type of a variable
/// 
/// ### Parameters:
/// - `_: &T` - The variable to print the type of
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


/// Return the type of a variable as a string
/// 
/// ### Parameters:
/// - `_: &T` - The variable to get the type of
/// 
/// ### Returns:
/// - [`String`] - The type of the variable
pub fn get_type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}
