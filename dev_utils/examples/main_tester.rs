use dev_utils::{app_dt, info};

fn main() {
    // 1. Display application information from Cargo.toml
    app_dt!(file!()); // This macro clears the screen and prints.

    app_dt!(file!(), "package" => ["license", "description"]); // Or select specific fields:

    // 2. Use the enhanced logger
    dev_utils::dlog::set_max_level(dev_utils::dlog::Level::Trace); // Set desired log level
    info!("Application started successfully!");
}
