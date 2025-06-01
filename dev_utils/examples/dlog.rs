use dev_utils::{__delay_ms, app_dt, datetime::*, dlog::*, format::*};

fn main() {
    app_dt!(file!());

    println!(
        "{}",
        "--- dlog Showcase ---".style(Style::Bold).color(CYAN)
    );

    // Initialize logging
    set_max_level(Level::Trace);

    // showcase_log_levels();
    // showcase_log_use_cases(); // * gen some delay's to simulate real-world scenarios
    // showcase_log_formatting();
    // showcase_datetime_features(); // Not very awesome... .__.
    showcase_log_performance();  // = 352.6482ms / 10000 logs (average of 10 runs)
}

fn showcase_log_levels() {
    println!(
        "\n{}",
        "Log Levels Demonstration:"
            .style(Style::Bold)
            .style(Style::Italic)
    );

    // Helper function to set log level, print color-coded messages, and log all levels
    fn demonstrate_log_level(level: Level, color: Color, description: &str) {
        println!(
            "\n{}",
            format!("Log Level: {}", description)
                .style(Style::Bold)
                .color(color)
        );
        set_max_level(level);
        log_all_levels();
    }

    // Function to log messages at all levels
    fn log_all_levels() {
        trace!("Most detailed level, useful for step-by-step debugging");
        debug!("Useful for diagnosing issues");
        info!("General operational messages about program execution");
        warn!("Something unexpected happened, but the program can still continue");
        error!("A serious problem occurred, indicating potential a failure");
    }

    // Demonstrate log levels with different settings
    demonstrate_log_level(
        Level::Trace,
        Color::new(180, 0, 158),
        "Trace (all levels visible)",
    );
    demonstrate_log_level(
        Level::Debug,
        Color::new(97, 214, 214),
        "Debug (Trace hidden, Debug and above visible)",
    );
    demonstrate_log_level(
        Level::Info,
        Color::new(22, 198, 12),
        "Info (Trace and Debug hidden, Info and above visible)",
    );
    demonstrate_log_level(
        Level::Warn,
        Color::new(245, 245, 57),
        "Warn (Only Warn and Error visible)",
    );
    demonstrate_log_level(
        Level::Error,
        Color::new(231, 72, 86),
        "Error (Only Error logs visible)",
    );

    // Restore Trace level at the end
    set_max_level(Level::Trace);
    println!(
        "\n{}",
        "Log Level restored to Trace."
            .style(Style::Bold)
            .color(GREEN)
    );
}

fn showcase_log_formatting() {
    println!(
        "\n{}",
        "Enhanced Log Formatting Features:"
            .style(Style::Bold)
            .style(Style::Italic)
    );

    info!("Standard log message");

    // Multi-line log for a simulated data structure
    let user_data = vec![
        ("UserID", "12345"),
        ("Username", "johndoe"),
        ("Email", "johndoe@example.com"),
        ("Role", "Admin"),
    ];

    debug!(
        "Logging multi-line structured data:\n{}",
        user_data
            .iter()
            .map(|(key, value)| format!("\t{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Log a long message split across multiple lines
    info!(
        "This is a long log message that spans multiple lines for better readability. \
        It demonstrates how long strings or messages can be split into readable chunks \
        without breaking the content flow."
    );

    // Log with colored and styled text
    let formatted_text = "Formatted".style(Style::Bold).color(GREEN);
    info!(
        "Logs can include {} and {} text",
        formatted_text,
        "styled".style(Style::Italic).color(MAGENTA)
    );

    // Log the current timestamp
    let now = DateTime::now();
    info!("Current timestamp: {}", now);

    // todo: Add full compatibility with thie multiline log
    let err_dt = vec![
        ("Code", "404"),
        ("Message", "Resource not found"),
        ("File", file!()),
    ];
    // iter over all the data on err_dt and apply the dim style to all the values
    let err_dt = err_dt
        .iter()
        .map(|(key, value)| (key, value.style(Style::Dim)))
        .collect::<Vec<_>>();

    // Multi-line error simulation
    error!(
        "Error on line: {}\n{}",
        line!(),
        err_dt
            .iter()
            .map(|(key, value)| format!("\t{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // todo: FIX THE ERRORS OCURRED WHEN HANDLING THE MULTILINE LOG...
    // todo: IT ALSO HAVE SOME ERROR IN WHICH THE STYLE IS APPLIED TO THE WHOLE STRING...
    // ^ In this case, the "Some new data:" is being styled as a whole string,
    // ^ not just the "Code: 200" and "Message: You got some successulf penchs"...
    // same as above but using the str in plain text
    info!(
        "Some new data:\n{}{}{}", // Added {} for the file part
        format!("\tCode: {}\n", "200".style(Style::Underline)), // Style only "200"
        format!("\tMessage: {}\n\t", "You got some successulf penchs".style(Style::Underline)), // Style only the message
        file!().style(Style::Bold)
    );
}

// = Time to log 10000 messages: 352.6482ms
// = Average time per log: 35.264µs
fn showcase_log_performance() {
    println!(
        "\n{}",
        "Log Performance:".style(Style::Bold).style(Style::Italic)
    );

    let iterations = 10000;
    let start = std::time::Instant::now();

    (0..iterations).for_each(|i| trace!("Performance test log {}", i));

    let duration = start.elapsed();
    println!("Time to log {} messages: {:?}", iterations, duration);
    println!("Average time per log: {:?}", duration / iterations as u32);
}

fn showcase_log_use_cases() {
    println!(
        "\n{}",
        "Practical Use Cases:"
            .style(Style::Bold)
            .style(Style::Italic)
    );

    // Simulating an application startup
    info!("Application starting up...");
    debug!("Initializing modules...");
    __delay_ms(500);
    info!("Database connection established");
    __delay_ms(300);
    // warn!("Config file not found, using default settings");
    __delay_ms(200);
    error!("Failed to load user preferences");
    info!("Application startup complete");

    // Simulating a function call
    debug!("Entering function process_data()");
    __delay_ms(100);
    trace!("Processing item 1 of 3");
    __delay_ms(50);
    trace!("Processing item 2 of 3");
    __delay_ms(50);
    trace!("Processing item 3 of 3");
    __delay_ms(100);
    debug!("Exiting function process_data()");

    info!("Data processing completed successfully");

    // same as above but now use a tuple to store the macro type, message, and delay, then iterate over it

    // logs.iter().for_each(|(log, msg, delay)| {
    //     log!("{}", msg);
    //     __delay_ms(*delay);
    // });
}

fn showcase_datetime_features() {
    println!(
        "\n{}",
        "DateTime Features:".style(Style::Bold).style(Style::Italic)
    );

    // Creating a DateTime instance
    let now = DateTime::now();
    info!("Current date and time: {}", now);

    // Creating a custom DateTime
    let custom_date = Date::new(2023, 12, 31).unwrap();
    let custom_time = Time::new(23, 59, 59).unwrap();
    let custom_datetime = DateTime {
        date: custom_date,
        time: custom_time,
    };
    info!("Custom DateTime: {}", custom_datetime);

    // Parsing a DateTime from a string
    let parsed_datetime: DateTime = "2023-05-01 12:34:56".parse().unwrap();
    info!("Parsed DateTime: {}", parsed_datetime);

    // DateTime from timestamp
    let from_timestamp = DateTime::from_timestamp(1682899200).unwrap();
    info!("DateTime from timestamp: {}", from_timestamp);

    // Demonstrating error handling
    match Date::new(2023, 2, 29) {
        // 29th Feb 2023 (not a leap year)
        Ok(date) => info!("Valid date: {:?}", date),
        Err(e) => println!("Invalid date: {}", e),
        // Err(e) => warn!("Invalid date: {}", e),
    }

    // Comparing DateTimes
    info!(
        "Comparing DateTimes: {} is earlier than {}",
        "2023-05-01 12:00:00".parse::<DateTime>().unwrap(),
        "2023-05-01 13:00:00".parse::<DateTime>().unwrap(),
    );

    // Demonstrating leap year
    let leap_year = 2024;
    info!(
        "Is {} a leap year? {}",
        leap_year,
        Date::is_leap_year(leap_year)
    );
}
