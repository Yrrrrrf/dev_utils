use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! define_levels {
    ($($level:ident => $value:expr, $color_code:expr),+ $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub enum Level {
            $($level = $value),+
        }

        impl fmt::Display for Level {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Level::$level => write!(f, stringify!($level))),+
                }
            }
        }

        impl Level {
            fn color_code(&self) -> &'static str {
                match self {
                    $(Level::$level => $color_code),+
                }
            }
        }
    };
}

define_levels! {
    Trace => 1, "\x1b[35;1m",  // Bold Magenta
    Debug => 2, "\x1b[36;1m",  // Bold Cyan
    Info  => 3, "\x1b[32;1m",  // Bold Green
    Warn  => 4, "\x1b[33;1m",  // Bold Yellow
    Error => 5, "\x1b[31;1m",  // Bold Red
}

static MAX_LOG_LEVEL: AtomicUsize = AtomicUsize::new(0);

pub fn set_max_level(level: Level) {
    MAX_LOG_LEVEL.store(level as usize, Ordering::SeqCst);
}

pub fn enabled(level: Level) -> bool {
    level as usize <= MAX_LOG_LEVEL.load(Ordering::Relaxed)
}


const LEVEL_WIDTH: usize = 0x05;  // * Just an unsigned integer w/ a fancy declaration

fn strip_ansi_escapes(src_str: &str) -> String {
    let mut result = String::with_capacity(src_str.len());
    let mut in_escape = false;
    src_str.chars().for_each(|c| match c {
        '\x1B' => in_escape = true,
        'm' if in_escape => in_escape = false,
        _ if !in_escape => result.push(c),
        _ => (),  // do nothing when the str is an escape sequence (e.g. \x1B[90m)
    });
    result
}

pub trait DlogStyle {
    fn format_log(&self, level: &Level, args: fmt::Arguments) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let secs = now.as_secs();
        let ms = now.subsec_millis();

        let (hr, min, sec) = (
            (secs / 3600) % 24,
            (secs / 60) % 60,
            secs % 60
        );

        let timestamp = format!("\x1b[90m[{hr:02}:{min:02}:{sec:02}.{ms:03}]\x1b[0m");
        
        let level_str = level.to_string();
        let level_str = self.level_color(level, 
            &format!("{level_str:>width$}", 
                width = LEVEL_WIDTH - ((LEVEL_WIDTH - level_str.len()) / 2)
        ));
        
        let prefix = format!("{} {} ", timestamp, level_str);
        let content_start = strip_ansi_escapes(&prefix).len();

        let mut output = String::new();
        for (i, line) in args.to_string().lines().enumerate() {
            match i {
                0 => output.push_str(&format!("{}{}", prefix, line)),
                _ => output.push_str(&format!("\n{}{} {line}", 
                    " ".repeat(content_start - 2),  // indent
                    self.level_color(level, "│")  // color the line
                )),
            }
        }
        output
    }

    fn level_color(&self, level: &Level, msg: &str) -> String {
        format!("{}{}\x1b[0m", level.color_code(), msg)
    }
}

pub struct DefaultDlogStyle;

impl DlogStyle for DefaultDlogStyle {}


pub fn log(style: &impl DlogStyle, level: Level, args: fmt::Arguments) {
    if enabled(level) {
        let log_message = style.format_log(&level, args);
        println!("{}", log_message);
    }
}


#[macro_export]
macro_rules! __dlog_internal {
    ($level:expr, $($arg:tt)+) => {
        $crate::dlog::log(&$crate::dlog::DefaultDlogStyle, $level, format_args!($($arg)+))
    };
}

#[macro_export] macro_rules! error { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Error, $($arg)+) }; }
#[macro_export] macro_rules! warn  { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Warn,  $($arg)+) }; }
#[macro_export] macro_rules! info  { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Info,  $($arg)+) }; }
#[macro_export] macro_rules! debug { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Debug, $($arg)+) }; }
#[macro_export] macro_rules! trace { ($($arg:tt)+) => { $crate::__dlog_internal!($crate::dlog::Level::Trace, $($arg)+) }; }


// todo: Improve this code by implemeneting some PROC MACRO
// todo: that will generate the following macros.
// todo: Because the code below is repetitive, so it can be generated.