use std::io::{Error, Result};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Format {
    /// escape character to start a sequence
    pub escape: String,

    /// prefix = escape + '['
    pub prefix: String,

    /// reset both style and color
    pub reset_all: String,

    /// text style
    pub style: Styles,

    /// text color
    pub color: Colors,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Styles {
    /// set style
    pub set: StyleCodes,

    /// reset style
    pub reset: StyleCodes,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StyleCodes {
    pub bold: String,
    pub dim: String,
    pub italic: String,
    pub underline: String,
    pub blink: String,
    pub reverse: String,
    pub invisible: String,
    pub strikethrough: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Colors {
    pub foreground: ColorCodes,
    pub background: ColorCodes,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ColorCodes {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub default: String,
}

#[allow(dead_code)]
impl Format {
    pub const ESC: &str = "\x1B";
    pub const PREFIX: &str = "\x1B[";
    pub const TERMINATION: &str = "m";
    pub const RESET_STYLE: &str = "\x1B[0m";

    pub fn new() -> Format {
        Format {
            escape: String::from(Self::ESC),
            prefix: String::from(Self::PREFIX),
            reset_all: String::from("0"),
            style: Styles {
                set: StyleCodes {
                    bold: String::from("1"),
                    dim: String::from("2"),
                    italic: String::from("3"),
                    underline: String::from("4"),
                    blink: String::from("5"),
                    reverse: String::from("7"),
                    invisible: String::from("8"),
                    strikethrough: String::from("9"),
                },
                reset: StyleCodes {
                    bold: String::from("22"),
                    dim: String::from("22"),
                    italic: String::from("23"),
                    underline: String::from("24"),
                    blink: String::from("25"),
                    reverse: String::from("27"),
                    invisible: String::from("28"),
                    strikethrough: String::from("29"),
                },
            },
            color: Colors {
                foreground: ColorCodes {
                    black: String::from("30"),
                    red: String::from("31"),
                    green: String::from("32"),
                    yellow: String::from("33"),
                    blue: String::from("34"),
                    magenta: String::from("35"),
                    cyan: String::from("36"),
                    white: String::from("37"),
                    default: String::from("39"),
                },
                background: ColorCodes {
                    black: String::from("40"),
                    red: String::from("41"),
                    green: String::from("42"),
                    yellow: String::from("43"),
                    blue: String::from("44"),
                    magenta: String::from("45"),
                    cyan: String::from("46"),
                    white: String::from("47"),
                    default: String::from("49"),
                },
            },
        }
    }

    pub fn generate_style_sequence(
        &self,
        style: Option<&Vec<String>>,
        foreground: Option<&String>,
        background: Option<&String>,
    ) -> Result<String> {
        if style.is_none() && foreground.is_none() && background.is_none() {
            return Err(Error::other("-- generate_style_sequence called without arguments"));
        }

        // assemble sequence if called with arguments
        let mut sequence = Vec::<String>::new();
        if let Some(item) = style {
            sequence.push_str(item);
            sequence.push(';');
        }
        if let Some(item) = foreground {
            sequence.push_str(item);
            sequence.push(';');
        }
        if let Some(item) = background {
            sequence.push_str(item);
            sequence.push(';');
        }
        sequence.pop(); // remove final semicolon
        sequence.push_str(Self::TERMINATION); // terminate sequence

        sequence
    }

    pub fn generate_rgb_sequence(&self, ) -> String {

        String::from("hi")
    }
}
