#![allow(unused)]

use std::{thread, time::Duration};

use dev_utils::{
    app_dt, conversion::base_change::*, debug, dlog::*, error, format::*, info, trace, warn
};

fn main() {
    app_dt!(file!(), "package" => ["license", "keywords"]);
    // test_logging();
    test_formatting();
    // test_base_conversion();
}


fn test_logging() {
    // Uncomment to set different log levels
    set_max_level(Level::Trace);    // trace all logs
    // set_max_level(Level::Debug); // debug & info & warn & error logs
    // set_max_level(Level::Info);  // info & warn & error logs
    // set_max_level(Level::Warn);  // warn & error logs
    // set_max_level(Level::Error); // error logs

    trace!("Trace log\nepilepsia\nesclavo del mal\ndaño cerebral\ndaño cerebral\npermanente");
    debug!("Debug log\n\nwith multiple lines");
    info!("Info log");
    info!("Info log");
    warn!("Warning log\n\nwith empty lines");
    error!("Error log\n\n\n\n\nwith many empty lines");
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




fn test_formatting() {
    print_styles();
    print_colors();
    print_gradients();
}

fn print_styles() {
    println!("\n--- Style Combinations ---\n");
    let styles = [Style::Bold, Style::Italic, Style::Underline, Style::Dim];
    let style_names = ["Bold", "Italic", "Underline", "Dim"];
    
    // Calculate maximum width for each column
    let max_width = style_names.iter().map(|name| name.len()).max().unwrap();
    
    // Print header
    print!("{:width$}", "", width = max_width + 2);
    for name in &style_names {
        print!("{:width$}", name, width = max_width + 2);
    }
    println!();
    
    // Print combinations
    for (i, style1) in styles.iter().enumerate() {
        print!("{:<width$}", style_names[i], width = max_width + 2);
        for style2 in &styles {
            let combined = "Sample".style(*style1).style(*style2);
            let padding = max_width + 2 - visual_length(&combined);
            print!("{}{}", combined, " ".repeat(padding));
        }
        println!();
    }
}

fn print_colors() {
    println!("\n--- Color Combinations (FG on BG) ---\n");
    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW, Color::MAGENTA, Color::CYAN];
    let color_names = ["Red", "Green", "Blue", "Yellow", "Magenta", "Cyan"];
    
    // Calculate maximum width for each column
    let max_width = color_names.iter().map(|name| name.len()).max().unwrap();
    
    // Print header
    print!("{:width$}", "", width = max_width + 2);
    for name in &color_names {
        print!("{:width$}", name, width = max_width + 2);
    }
    println!();
    
    // Print combinations
    for (i, fg_color) in colors.iter().enumerate() {
        print!("{:<width$}", color_names[i], width = max_width + 2);
        for bg_color in &colors {
            let combined = "Sample".color(*fg_color).on_color(*bg_color);
            let padding = max_width + 2 - visual_length(&combined);
            print!("{}{}", combined, " ".repeat(padding));
        }
        println!();
    }
}

fn print_gradients() {
    println!("\n--- Gradient Demonstrations ---\n");

    fn create_gradient(start: Color, end: Color, steps: usize) -> String {
        (0..steps).map(|i| {
            let t = i as f32 / (steps - 1) as f32;
            let r = (start.to_rgb().0 as f32 * (1.0 - t) + end.to_rgb().0 as f32 * t) as u8;
            let g = (start.to_rgb().1 as f32 * (1.0 - t) + end.to_rgb().1 as f32 * t) as u8;
            let b = (start.to_rgb().2 as f32 * (1.0 - t) + end.to_rgb().2 as f32 * t) as u8;
            "■".color(Color::from((r, g, b)))
        }).collect()
    }

    fn create_rectangular_gradient(width: usize, height: usize) -> String {
        let mut result = String::new();
        for y in 0..height {
            for x in 0..width {
                let r = (255.0 * x as f32 / width as f32) as u8;
                let g = (255.0 * y as f32 / height as f32) as u8;
                let b = (255.0 * (1.0 - (x as f32 / width as f32).max(y as f32 / height as f32))) as u8;
                let color = Color::from((r, g, b));
                result.push_str(&"██".color(color));
            }
            result.push('\n');
        }
        result
    }

    println!("Linear Gradient (Red to Blue):");
    println!("{}\n", create_gradient(Color::RED, Color::BLUE, 15));

    println!("Rectangular Gradient:");
    // println!("{}", create_rectangular_gradient(15, 8));
    println!("{}", create_rectangular_gradient(32, 32));
}
