//! This is an experimental module for logging.
//! 
//! It uses the tracing crate for logging.
//! 
//! Used to remove the dependency on the `log` crate.

// pub fn log_init() {
//     log_init_with_level(Level::INFO);
// }

// pub enum Level {
//     TRACE,
//     DEBUG,
//     INFO,
//     WARN,
//     ERROR
// }

// pub fn log_init_with_level(level: Level) {
//     struct LocalTimer;
//     impl tracing_subscriber::fmt::time::FormatTime for LocalTimer {
//         fn format_time(
//             &self,
//             w: &mut tracing_subscriber::fmt::format::Writer<'_>,
//         ) -> std::fmt::Result {
//             write!(w, "{}", chrono::Local::now().format("%F %T%.3f"))
//         }
//     }
//     let format = tracing_subscriber::fmt::format()
//         .with_level(true)
//         .with_target(true)
//         .with_line_number(true)
//         .with_thread_ids(true)
//         .with_thread_names(false)
//         .with_timer(LocalTimer);

//     let log_level = match level {
//         Level::TRACE => tracing::Level::TRACE,
//         Level::DEBUG => tracing::Level::DEBUG,
//         Level::INFO => tracing::Level::INFO,
//         Level::WARN => tracing::Level::WARN,
//         Level::ERROR => tracing::Level::ERROR,
//     };

//     match tracing_subscriber::fmt()
//         .with_max_level(log_level)
//         .with_writer(std::io::stdout)
//         .with_ansi(true)
//         .event_format(format)
//         .try_init()
//     {
//         Ok(_) => {}
//         Err(_) => {}
//     };
// }