use std::io::{Error, Result};

#[derive(Debug, Clone)]
pub struct Format {
    // reset both style and color
    reset_all: String,

    // text style
    style: Styles,

    // text color
    color: Colors,
}

#[derive(Debug, Clone)]
pub struct Styles {
    // set style
    set: StyleCodes,

    // reset style
    reset: StyleCodes,
}

#[derive(Debug, Clone)]
pub struct StyleCodes {
    bold: String,
    dim: String,
    italic: String,
    underline: String,
    blink: String,
    reverse: String,
    invisible: String,
    strikethrough: String,
}

#[derive(Debug, Clone)]
pub struct Colors {
    foreground: ColorCodes,
    background: ColorCodes,
}

#[derive(Debug, Clone)]
pub struct ColorCodes {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
    default: String,
}

impl Format {
    const ESC: &str = "\x1B";
    const PREFIX: &str = "\x1B[";

    pub fn new() -> Format {
        Format {
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

}
