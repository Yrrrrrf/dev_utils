#![allow(unused)]


use std::{
    thread,
    time::Duration,
};

// todo: Define the prelude in the dev_utils crate
use dev_utils::{
    // todo: Add this stuff to the dev_utils::prelude
    app_dt, error, warn, info, debug, trace,
    dlog::*,
    format::*,
};

fn main() {
    app_dt!(file!());  // Print application data (name, version, authors, etc.)
    // app_dt!(file!(),
    //     "dependencies" => ["serde", "tokio"]
    // );

    // set_max_level(Level::Trace);  // Set Trace level (lowest level)
    // set_max_level(Level::Warn);  // Set Trace level (lowest level)
    // set_max_level(Level::Error);  // Set Trace level (lowest level)
    trace!("Some data!\nepilepsia\nesclavo del mal\ndano cerebral\ndano cerebral\npermanente");

    // __delay(1000);

    trace!("Some \n\ndata! {}", "penchs");  // same as println!() or print!() macro's
    debug!("Some \n\ndata!");
    debug!("Some data!");
    debug!("Some data!");
    debug!("Some data!");
    debug!("Some data!");
    debug!("Some data!");
    info!("Some d\n\nata!");
    warn!("Some d\n\nata!");
    error!("Some data!\n\n\n\n\nhola");

    let s = String::from("dsasadsad");

    // let a = "sda".green();

    let s = s + "dsadsadsa";

    
    showcase_formatting();

}


fn __delay(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("\t----Delay---- ({ms} ms)");
}


// #[test]
fn showcase_formatting() {
    // Helper function to print both formatted and stripped versions
    fn print_showcase(description: &str, formatted: &str) {
        println!("\n{}", description);
        println!("Formatted  : {}", formatted);
        println!("Stripped   : {}", strip_ansi_codes(formatted));
        println!("Visual Len : {}", visual_length(formatted));
    }

    // Basic colors
    print_showcase("Basic colors:", 
        &format!("{} {} {} {}",
            "Red".color(Color::RED),
            "Green".color(Color::GREEN),
            "Blue".color(Color::BLUE),
            "Yellow".color(Color::YELLOW)
        )
    );

    // Background colors
    print_showcase("Background colors:", 
        &format!("{} {} {}",
            "Red BG".on_color(Color::RED),
            "Green BG".on_color(Color::GREEN),
            "Blue BG".on_color(Color::BLUE)
        )
    );

    // Styles
    print_showcase("Styles:", 
        &format!("{} {} {} {}",
            "Bold".style(Style::Bold),
            "Italic".style(Style::Italic),
            "Underline".style(Style::Underline),
            "Blink".style(Style::Blink)
        )
    );

    // Complex formatting
    print_showcase("Complex formatting:",
        &"Rainbow".color(Color::RED)
            .on_color(Color::YELLOW)
            .style(Style::Bold)
    );

    // Custom colors
    let pastel_pink = Color::from((255, 182, 193));
    let light_blue = Color::from((173, 216, 230));
    print_showcase("Custom colors:",
        &format!("{} {}",
            "Pastel Pink".color(pastel_pink),
            "Light Blue".color(light_blue)
        )
    );

    // Gradients (simulated)
    let gradient = (0..=5).map(|i| {
        let color = Color::from((255, i * 51, 255 - i * 51));
        "█".repeat(5).color(color)
    }).collect::<Vec<_>>().join("");
    print_showcase("Gradient (simulated):", &gradient);

    // Nested formatting
    print_showcase("Nested formatting:",
        &format!("{}{}{}",
            "Red ".color(Color::RED).style(Style::Italic).on_color(Color::WHITE),
            "Green ".color(Color::GREEN).on_color(Color::YELLOW),
            "Blue".color(Color::BLUE).style(Style::Bold)
        )
    );

}
