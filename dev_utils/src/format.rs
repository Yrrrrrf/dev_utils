use std::fmt;



// Define a struct for RGB colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB(pub u8, pub u8, pub u8);

// Macro to create Color enum and implement color codes
macro_rules! create_color_enum {
    ($($name:ident => ($r:expr, $g:expr, $b:expr)),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Color {
            $($name,)*
            Custom(RGB),
        }

        impl Color {
            pub fn to_rgb(&self) -> RGB {
                match self {
                    $(Color::$name => RGB($r, $g, $b),)*
                    Color::Custom(rgb) => *rgb,
                }
            }

            pub fn as_fg(&self) -> String {
                let RGB(r, g, b) = self.to_rgb();
                format!("\x1b[38;2;{};{};{}m", r, g, b)
            }

            pub fn as_bg(&self) -> String {
                let RGB(r, g, b) = self.to_rgb();
                format!("\x1b[48;2;{};{};{}m", r, g, b)
            }
        }

        impl From<RGB> for Color {
            fn from(rgb: RGB) -> Self {
                Color::Custom(rgb)
            }
        }
        
        impl From<(u8, u8, u8)> for Color {
            fn from(rgb: (u8, u8, u8)) -> Self {
                Color::Custom(RGB(rgb.0, rgb.1, rgb.2))
            }
        }

        $(pub const $name: Color = Color::$name;)*
    };
}

// Create Color enum and implement color codes
create_color_enum! {
    BLACK   => (  0,   0,   0),  // 000
    BLUE    => (  0,   0, 255),  // 00F
    GREEN   => (  0, 255,   0),  // 0F0
    CYAN    => (  0, 255, 255),  // 0FF
    RED     => (255,   0,   0),  // F00
    MAGENTA => (255,   0, 255),  // F0F
    YELLOW  => (255, 255,   0),  // FF0
    WHITE   => (255, 255, 255),  // FFF
    // * define any custom colors here...

    // todo: Improve this macro to now be able to:
    // todo: - Just call the macro with the desired RGB values
    // todo: - Automatically create the Color enum and implement the color codes
}


// Macro to create Style enum and implement style codes
macro_rules! create_style_enum {
    ($(($style:ident, $code:expr)),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Style {$($style,)*}

        impl Style {
            pub fn code(&self) -> String {
                match self {$(Style::$style => format!("\x1b[{}m", $code),)*}
            }
        }
    };
}

// Create Style enum and implement style codes
create_style_enum! {
    (Bold, 1),  // 1
    (Dim, 2),  // 1
    (Italic, 3),  // 1
    (Underline, 4),  // 1
    // (Blink, 5),  // 0 (does not work in most terminals)
    // (Reverse, 7),  // 0 (does not work in most terminals)
    (Hidden, 8),  // 1
}

// Trait for styling
pub trait Stylize {
    fn color(&self, color: Color) -> String;
    fn on_color(&self, color: Color) -> String;
    fn style(&self, style: Style) -> String;
}

// Macro to implement Stylize for both &str and String
macro_rules! impl_stylize {
    ($($t:ty)*) => ($(
        impl Stylize for $t {
            fn color(&self, color: Color) -> String {format!("{}{}\x1b[0m", color.as_fg(), self)}
            fn on_color(&self, color: Color) -> String {format!("{}{}\x1b[0m", color.as_bg(), self)}
            fn style(&self, style: Style) -> String {format!("{}{}\x1b[0m", style.code(), self)}
        }
    )*)
}

impl_stylize! { str String }


pub fn strip_ansi_codes(s: &str) -> String {
    #[derive(Clone, Copy)]
    enum State { Normal, Escape, CSI }

    // THIS FSM is used to strip ANSI escape codes from the given string
    // It scans through the characters and removes the ANSI codes
    s.chars()
        .scan(State::Normal, |state, c| {
            match (*state, c) {
                (State::Normal, '\x1B') => {
                    *state = State::Escape;
                    Some(None)
                },
                (State::Escape, '[') => {
                    *state = State::CSI;
                    Some(None)
                },
                (State::Escape, _) => {
                    *state = State::Normal;
                    Some(Some('\x1B'))
                },
                (State::CSI, 'm') => {
                    *state = State::Normal;
                    Some(None)
                },
                (State::CSI, '0'..='9') | (State::CSI, ';') => Some(None),
                (State::CSI, _) => {
                    *state = State::Normal;
                    Some(Some(c))
                },
                (State::Normal, c) => Some(Some(c)),
            }
        })
        .flatten()
        .collect()
}

pub fn visual_length(s: &str) -> usize {
    strip_ansi_codes(s).chars().count()
}






#[cfg(test)]
mod tests {
    use super::*;  // use the contents of the parent module

    #[test]
    fn test_color_creation() {
        assert_eq!(Color::RED.to_rgb(), RGB(255, 0, 0));
        assert_eq!(Color::from((128, 64, 32)).to_rgb(), RGB(128, 64, 32));
    }

    #[test]
    fn test_color_codes() {
        assert_eq!(Color::BLUE.as_fg(), "\x1b[38;2;0;0;255m");
        assert_eq!(Color::GREEN.as_bg(), "\x1b[48;2;0;255;0m");
        assert_eq!(Color::from((128, 64, 32)).as_fg(), "\x1b[38;2;128;64;32m");
    }

    #[test]
    fn test_style_codes() {
        assert_eq!(Style::Bold.code(), "\x1b[1m");
        assert_eq!(Style::Underline.code(), "\x1b[4m");
    }

    #[test]
    fn test_stylize_trait() {
        let text = "Test";
        assert_eq!(text.color(Color::RED), "\x1b[38;2;255;0;0mTest\x1b[0m");
        assert_eq!(text.on_color(Color::BLUE), "\x1b[48;2;0;0;255mTest\x1b[0m");
        assert_eq!(text.style(Style::Bold), "\x1b[1mTest\x1b[0m");
    }

    #[test]
    fn test_visual_length() {
        let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGREEN\x1b[0m";
        assert_eq!(visual_length(colored_text), 9); // "RED GREEN".len()
    }

    #[test]
    fn test_complex_formatting() {
        let text = "Complex";
        let formatted = text.color(Color::RED).on_color(Color::WHITE).style(Style::Bold);
        let stripped = strip_ansi_codes(&formatted);
        assert_eq!(stripped, "Complex");
        assert!(formatted.contains("\x1b[38;2;255;0;0m")); // Red foreground
        assert!(formatted.contains("\x1b[48;2;255;255;255m")); // White background
        assert!(formatted.contains("\x1b[1m")); // Bold
        assert!(formatted.ends_with("\x1b[0m")); // Reset at the end
    }

    #[test]
    fn test_custom_color() {
        let custom_color = Color::from((123, 45, 67));
        assert_eq!(custom_color.as_fg(), "\x1b[38;2;123;45;67m");
    }

    #[test]
    fn test_strip_ansi_codes() {
        let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGreen\x1b[0m";
        assert_eq!(strip_ansi_codes(colored_text), "Red Green");
        
        // Test with multiple consecutive ANSI codes
        let multi_code = "\x1b[31;1mBold Red\x1b[0m";
        assert_eq!(strip_ansi_codes(multi_code), "Bold Red");
    }
    
    #[test]
    fn test_incomplete_ansi_sequence() {
        let incomplete = "Normal \x1b[31m Red \x1b[ Incomplete";
        assert_eq!(strip_ansi_codes(incomplete), "Normal  Red  Incomplete");
        
        // Test with incomplete sequence at the end
        let incomplete_end = "Text with incomplete sequence\x1b[";
        assert_eq!(strip_ansi_codes(incomplete_end), "Text with incomplete sequence");
    }
}

