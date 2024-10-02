use std::fmt;



// Define a struct for RGB colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB(pub u8, pub u8, pub u8);

// Macro to create Color enum and implement color codes
macro_rules! create_color_enum {
    ($(($name:ident, $r:expr, $g:expr, $b:expr)),* $(,)?) => {
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

            pub fn fg_code(&self) -> String {
                let RGB(r, g, b) = self.to_rgb();
                format!("\x1b[38;2;{};{};{}m", r, g, b)
            }

            pub fn bg_code(&self) -> String {
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

        $(
            pub const $name: Color = Color::$name;
        )*
    };
}

// Create Color enum and implement color codes
create_color_enum! {
    (BLACK,     0,   0,   0),  // 000
    (BLUE,      0,   0, 255),  // 00F
    (GREEN,     0, 255,   0),  // 0F0
    (CYAN,      0, 255, 255),  // 0FF
    (RED,     255,   0,   0),  // F00
    (MAGENTA, 255,   0, 255),  // F0F
    (YELLOW,  255, 255,   0),  // FF0
    (WHITE,   255, 255, 255),  // FFF
}


// Macro to create Style enum and implement style codes
macro_rules! create_style_enum {
    ($(($style:ident, $code:expr)),* $(,)?) => {
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
    (Bold, 1),
    (Dim, 2),
    (Italic, 3),
    (Underline, 4),
    (Blink, 5),
    (Reverse, 7),
    (Hidden, 8),
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
            fn color(&self, color: Color) -> String {format!("{}{}\x1b[0m", color.fg_code(), self)}
            fn on_color(&self, color: Color) -> String {format!("{}{}\x1b[0m", color.bg_code(), self)}
            fn style(&self, style: Style) -> String {format!("{}{}\x1b[0m", style.code(), self)}
        }
    )*)
}

impl_stylize! { str String }


pub fn strip_ansi_codes(s: &str) -> String {
    #[derive(Clone, Copy)]
    enum State { Normal, Escape, CSI }
    
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
