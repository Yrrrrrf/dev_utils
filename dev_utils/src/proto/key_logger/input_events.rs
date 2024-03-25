#![allow(dead_code)]  // Allow dead code in a file or globally


use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)

use device_query::{DeviceQuery, DeviceState, MouseState, Keycode, device_state};  // device_query library


/// Runs the module
pub fn run() {
    println!("Module path: {}\n\n", module_path!());
    print_event();
}


/// Prints always a key is pressed or unpressed
pub fn print_event() {
    let mut old_keys: Vec<Keycode> = Vec::new();  // create a new vector of Keycode (empty)
    let mut mouse: MouseState = DeviceState::new().get_mouse();
    let mut old_mouse: MouseState = mouse.clone();

    loop {
        // ? Keyboard events
        let new_keys: Vec<Keycode> = DeviceState::new().get_keys(); // get the current pressed keys
        if new_keys.contains(&Keycode::Escape) {break;}  // if escape is pressed, then break the loop

        new_keys.iter().for_each(|key| if !old_keys.contains(key) { println!("↓ {:?}", key); });  // print when a key is   PRESSED
        old_keys.iter().for_each(|key| if !new_keys.contains(key) { println!("↑ {:?}", key); });  // print when a key is UNPRESSED

        // ? Mouse events
        mouse = DeviceState::new().get_mouse();  // get the current mouse state
        print_mouse_events(&mouse, &old_mouse);

        // ? Update the old states
        old_mouse = mouse.clone();  // update the mouse state
        old_keys = new_keys.clone();  // update the old keys
    }
}


/// Prints the mouse events
pub fn print_mouse_events(mouse: &MouseState, mut old_mouse: &MouseState) {
    if mouse.coords.0 != old_mouse.coords.0 || mouse.coords.1 != old_mouse.coords.1 {print!("\r{:>6},{:>6}", mouse.coords.0, mouse.coords.0);}  // print when the mouse moves
    mouse.button_pressed.iter().enumerate().for_each(|(i, button)| if !old_mouse.button_pressed[i] && *button { println!("↓ {:?}", i); });  // print when a mouse button is   PRESSED
    old_mouse.button_pressed.iter().enumerate().for_each(|(i, button)| if !mouse.button_pressed[i] && *button { println!("↑ {:?}", i); });  // print when a mouse button is UNPRESSED
}
