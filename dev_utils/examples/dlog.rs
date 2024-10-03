use dev_utils::{
    app_dt, conversion::datetime::{Date, DateTime, Time}, debug, dlog::*, error, format::*, info, trace, warn
};
use std::{str::FromStr, thread, time::Duration};

fn main() {
    app_dt!(file!());
    
    println!("\n{}", "--- dlog Showcase ---".style(Style::Bold).color(Color::CYAN));

    // Initialize logging
    set_max_level(Level::Trace);

    showcase_log_levels();
    // showcase_log_use_cases();  // * gen some delay's to simulate real-world scenarios
    showcase_log_formatting();
    // showcase_datetime_features();  // Not very awesome... .__. 
    // showcase_log_performance();  // = 352.6482ms / 10000 logs (average of 10 runs)
}

fn showcase_log_levels() {
    println!("\n{}", "Log Levels Demonstration:".style(Style::Bold).style(Style::Italic));

    // Helper function to set log level, print color-coded messages, and log all levels
    fn demonstrate_log_level(level: Level, color: Color, description: &str) {
        println!("\n{}", format!("Log Level: {}", description).style(Style::Bold).color(color));
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
    demonstrate_log_level(Level::Trace, Color::Custom(RGB(180, 0, 158)), "Trace (all levels visible)");
    demonstrate_log_level(Level::Debug, Color::Custom(RGB(97, 214, 214)), "Debug (Trace hidden, Debug and above visible)");
    demonstrate_log_level(Level::Info,  Color::Custom(RGB(22, 198, 12)), "Info (Trace and Debug hidden, Info and above visible)");
    demonstrate_log_level(Level::Warn,  Color::Custom(RGB(245, 245, 57)), "Warn (Only Warn and Error visible)");
    demonstrate_log_level(Level::Error, Color::Custom(RGB(231, 72, 86)), "Error (Only Error logs visible)");

    // Restore Trace level at the end
    set_max_level(Level::Trace);
    println!("\n{}", "Log Level restored to Trace.".style(Style::Bold).color(Color::GREEN));
}


fn showcase_log_formatting() {
    println!("\n{}", "Enhanced Log Formatting Features:".style(Style::Bold).style(Style::Italic));

    info!("Standard log message");

    // Multi-line log for a simulated data structure
    let user_data = vec![
        ("UserID", "12345"),
        ("Username", "johndoe"),
        ("Email", "johndoe@example.com"),
        ("Role", "Admin"),
    ];

    debug!("Logging multi-line structured data:\n{}",
        user_data.iter().map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<_>>().join("\n")
    );

    // Log a long message split across multiple lines
    info!(
        "This is a long log message that spans multiple lines for better readability. \
         It demonstrates how long strings or messages can be split into readable chunks \
         without breaking the content flow."
    );

    // Log with colored and styled text
    let formatted_text = "Formatted".style(Style::Bold).color(Color::GREEN);
    info!("Logs can include {} and {} text", formatted_text, "styled".style(Style::Italic).color(Color::MAGENTA));

    // Log the current timestamp
    let now = DateTime::now();
    info!("Current timestamp: {}", now);

    // Multi-line error simulation
    error!(
        "Error encountered:\n\
         \tCode: 404\n\
         \tMessage: Resource not found\n\
         \tFile: {}\n\
         \tLine: {}",
        file!(),
        line!()
    );
}


// = Time to log 10000 messages: 352.6482ms
// = Average time per log: 35.264µs
fn showcase_log_performance() {
    println!("\n{}", "Log Performance:".style(Style::Bold).style(Style::Italic));
    
    let iterations = 10000;
    let start = std::time::Instant::now();

    (0..iterations).for_each(|i| trace!("Performance test log {}", i));
    
    let duration = start.elapsed();
    println!("Time to log {} messages: {:?}", iterations, duration);
    println!("Average time per log: {:?}", duration / iterations as u32);
}

fn showcase_log_use_cases() {
    println!("\n{}", "Practical Use Cases:".style(Style::Bold).style(Style::Italic));
    
    // Simulating an application startup
    info!("Application starting up...");
    debug!("Initializing modules...");
    thread::sleep(Duration::from_millis(500));
    info!("Database connection established");
    thread::sleep(Duration::from_millis(300));
    warn!("Config file not found, using default settings");
    thread::sleep(Duration::from_millis(200));
    error!("Failed to load user preferences");
    info!("Application startup complete");

    // Simulating a function call
    debug!("Entering function process_data()");
    thread::sleep(Duration::from_millis(100));
    trace!("Processing item 1 of 3");
    thread::sleep(Duration::from_millis(50));
    trace!("Processing item 2 of 3");
    thread::sleep(Duration::from_millis(50));
    trace!("Processing item 3 of 3");
    thread::sleep(Duration::from_millis(100));
    debug!("Exiting function process_data()");

    info!("Data processing completed successfully");
}


fn showcase_datetime_features() {
    println!("\n{}", "DateTime Features:".style(Style::Bold).style(Style::Italic));

    // Creating a DateTime instance
    let now = DateTime::now();
    info!("Current date and time: {}", now);

    // Creating a custom DateTime
    let custom_date = Date::new(2023, 12, 31).unwrap();
    let custom_time = Time::new(23, 59, 59).unwrap();
    let custom_datetime = DateTime { date: custom_date, time: custom_time };
    info!("Custom DateTime: {}", custom_datetime);

    // Parsing a DateTime from a string
    let parsed_datetime = DateTime::from_str("2023-05-01 12:34:56").unwrap();
    info!("Parsed DateTime: {}", parsed_datetime);

    // DateTime from timestamp
    let from_timestamp = DateTime::from_timestamp(1682899200).unwrap();
    info!("DateTime from timestamp: {}", from_timestamp);

    // Demonstrating error handling
    match Date::new(2023, 2, 29) {
        Ok(date) => info!("Valid date: {:?}", date),
        Err(e) => warn!("Invalid date: {}", e),
    }

    // Comparing DateTimes
    let dt1 = DateTime::from_str("2023-05-01 12:00:00").unwrap();
    let dt2 = DateTime::from_str("2023-05-01 13:00:00").unwrap();
    info!("Comparing DateTimes: {} is earlier than {}", dt1, dt2);

    // Demonstrating leap year
    let leap_year = 2024;
    info!("Is {} a leap year? {}", leap_year, Date::is_leap_year(leap_year));
}
