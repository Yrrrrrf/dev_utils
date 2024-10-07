use dev_utils::{app_dt, format::*};


fn main() {
    app_dt!(file!());
    // * Showcasing basic formatting capabilities
    print_styles();
    print_colors();
    print_gradients();
}

fn print_styles() {
    println!("\n--- Style Combinations ---\n");
    let styles = [Style::Bold, Style::Italic, Style::Underline, Style::Dim];
    let style_names = styles.iter().map(|style| format!("{:?}", style)).collect::<Vec<_>>();

    let max_width = style_names.iter().map(|name| name.len()).max().unwrap();
    
    print!("{:width$}", "", width = max_width + 2);
    for name in &style_names {
        print!("{:width$}", name, width = max_width + 2);
    }
    println!();
    
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
    let colors = [BLUE, GREEN, CYAN, RED, MAGENTA, YELLOW, WHITE];
    let color_names = ["Blue", "Green", "Cyan", "Red", "Magenta", "Yellow", "White"];

    let max_width = color_names.iter().map(|name| name.len()).max().unwrap();
    
    print!("{:width$}", "", width = max_width + 2);
    for name in &color_names {
        print!("{:width$}", name, width = max_width + 2);
    }
    println!();
    
    for (i, fg_color) in colors.iter().enumerate() {
        print!("{:<width$}", color_names[i], width = max_width + 2);
        for bg_color in &colors {
            let combined = " Sample ".color(*fg_color).on_color(*bg_color);
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
                result.push_str(&"██".color(Color::from((
                    // * the output will look like:
                    // .     LU: (Red)
                    // *     RU: (Green)
                    // ?     LD: (Blue)
                    // *     RD: (R+G)
                    (255.0 * (1.0 - (x as f32 / width as f32).max(y as f32 / height as f32))) as u8,                
                    (255.0 * y as f32 / height as f32) as u8,
                    (255.0 * x as f32 / width as f32) as u8,
                ))));
            }
            result.push('\n');
        }
        result
    }

    println!("Linear Gradient (Red to Blue):");
    println!("{}\n", create_gradient(RED, BLUE, 15));

    println!("Rect Gradient:");
    println!("{}", create_rectangular_gradient(32, 16));
}
