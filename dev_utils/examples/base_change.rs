use dev_utils::{app_dt, base_change::convert_base, format::*};

fn main() {
    app_dt!(file!());
    println!(
        "\n{}",
        "--- Base Change Module Showcase ---"
            .style(Style::Bold)
            .color(CYAN)
    );

    // * PASSED:
    showcase_basic_conversions();
    showcase_error_handling();
    showcase_performance();

    // . W/ ERROR:
    // showcase_advanced_conversions();
    // showcase_precision();
}

fn showcase_basic_conversions() {
    println!(
        "\n{}",
        "Basic Conversions:".style(Style::Bold).style(Style::Italic)
    );

    let conversions = [
        ("1010", 2, 10, "Binary to Decimal"),
        ("42", 10, 2, "Decimal to Binary"),
        ("255", 10, 16, "Decimal to Hexadecimal"),
        ("FF", 16, 10, "Hexadecimal to Decimal"),
        ("1010", 2, 16, "Binary to Hexadecimal"),
        ("A", 16, 2, "Hexadecimal to Binary"),
    ];

    for (input, from_base, to_base, description) in conversions.iter() {
        match convert_base(input, *from_base, *to_base) {
            Ok(result) => println!(
                "{}: {} (base {}) -> {} (base {})",
                description.color(GREEN),
                input.style(Style::Bold),
                from_base,
                result.style(Style::Bold),
                to_base
            ),
            Err(e) => println!("{}: {} (Error: {:?})", description.color(RED), input, e),
        }
    }
}

fn showcase_advanced_conversions() {
    println!(
        "\n{}",
        "Advanced Conversions:"
            .style(Style::Bold)
            .style(Style::Italic)
    );

    let conversions = [
        ("0.5", 10, 2, "Decimal fraction to Binary"),
        ("3.14159", 10, 16, "Decimal PI to Hexadecimal"),
        ("HelloWorld", 62, 10, "Base62 to Decimal"),
        ("239032307299047885", 10, 62, "Large Decimal to Base62"),
        ("1000000", 10, 36, "Decimal to Base36"),
    ];

    for (input, from_base, to_base, description) in conversions.iter() {
        match convert_base(input, *from_base, *to_base) {
            Ok(result) => println!(
                "{}: {} (base {}) -> {} (base {})",
                description.color(BLUE),
                input.style(Style::Bold),
                from_base,
                result.style(Style::Bold),
                to_base
            ),
            Err(e) => println!("{}: {} (Error: {:?})", description.color(RED), input, e),
        }
    }
}

fn showcase_error_handling() {
    println!(
        "\n{}",
        "Error Handling:".style(Style::Bold).style(Style::Italic)
    );

    let error_cases = [
        ("10", 1, 10, "Invalid input base"),
        ("10", 10, 63, "Invalid output base"),
        ("2", 2, 10, "Invalid digit for base"),
        ("G", 16, 10, "Invalid hexadecimal digit"),
        ("1.2.3", 10, 2, "Invalid number format"),
    ];

    for (input, from_base, to_base, description) in error_cases.iter() {
        match convert_base(input, *from_base, *to_base) {
            Ok(_) => println!("{}: Unexpectedly succeeded", description.color(YELLOW)),
            Err(e) => println!("{}: {} (Error: {:?})", description.color(RED), input, e),
        }
    }
}

fn showcase_performance() {
    println!(
        "\n{}",
        "Performance Test:".style(Style::Bold).style(Style::Italic)
    );

    let iterations = 100_000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = convert_base("123456789", 10, 16);
    }

    let duration = start.elapsed();
    println!("Time to perform {} op: {:?}", iterations, duration);
    println!("Average time per op: {:?}", duration / iterations as u32);
    println!(
        "Operations per sec: {:.2}",
        iterations as f64 / duration.as_secs_f64()
    );
}

// You can add this function if you want to demonstrate the precision of the conversions
fn showcase_precision() {
    println!(
        "\n{}",
        "Precision Demonstration:"
            .style(Style::Bold)
            .style(Style::Italic)
    );

    let precision_tests = [
        "0.1234567890123456789",
        "3.141592653589793",
        "2.718281828459045",
    ];

    for &test in precision_tests.iter() {
        let binary = convert_base(test, 10, 2).unwrap();
        let back_to_decimal = convert_base(&binary, 2, 10).unwrap();

        println!("Original: {}", test.color(CYAN));
        println!("To Binary: {}", binary.color(MAGENTA));
        println!("Back to Decimal: {}", back_to_decimal.color(GREEN));
        println!();
    }
}
