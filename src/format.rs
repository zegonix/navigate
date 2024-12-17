#![allow(dead_code)]

use std::io::{Result, Error};

pub const ESC: &str = "\x1B";
pub const PREFIX: &str = "\x1B[";
pub const RESET_ARG: &str = "0";
pub const TERMINATION: &str = "m";
pub const RESET_SEQ: &str = "\x1B[0m";
pub const FG: ColorContext = ColorContext::Foreground;
pub const BG: ColorContext = ColorContext::Background;

pub const STYLES: Styles = Styles {
    set: StyleCodes {
        bold: "1",
        dim: "2",
        italic: "3",
        underline: "4",
        blink: "5",
        reverse: "7",
        invisible: "8",
        strikethrough: "9",
    },
    reset: StyleCodes {
        bold: "22",
        dim: "22",
        italic: "23",
        underline: "24",
        blink: "25",
        reverse: "27",
        invisible: "28",
        strikethrough: "29",
    },
};

pub const COLORS: Colors = Colors {
    fg: ColorCodes {
        black: "30",
        red: "31",
        green: "32",
        yellow: "33",
        blue: "34",
        magenta: "35",
        cyan: "36",
        white: "37",
        extended: "38",
        default: "39",
    },
    bg: ColorCodes {
        black: "40",
        red: "41",
        green: "42",
        yellow: "43",
        blue: "44",
        magenta: "45",
        cyan: "46",
        white: "47",
        extended: "48",
        default: "49",
    },
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Styles {
    pub set: StyleCodes,
    pub reset: StyleCodes,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StyleCodes {
    pub bold: &'static str,
    pub dim: &'static str,
    pub italic: &'static str,
    pub underline: &'static str,
    pub blink: &'static str,
    pub reverse: &'static str,
    pub invisible: &'static str,
    pub strikethrough: &'static str,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Colors {
    pub fg: ColorCodes,
    pub bg: ColorCodes,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ColorCodes {
    pub black: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
    pub white: &'static str,
    pub extended: &'static str,
    pub default: &'static str,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ColorContext {
    Foreground,
    Background,
}

/// prepends input with style string and appends the reset sequence at the end
pub fn apply_format(input: &str, style: &str) -> String {
    format!("{}{}{}", style, input, RESET_SEQ)
}

/// generates a common style sequence of format
/// `\x1B[<styles>;<foreground-color>;<background-color>m`
/// all elements are optional, if none is supplied the function returns an error
pub fn generate_style_sequence(
    style: Option<Vec<&str>>,
    foreground: Option<&str>,
    background: Option<&str>,
) -> String {
    // assemble sequence as vector of string
    // this way the semicolons to separate arguments
    // can be inserted with 'join()'
    let mut sequence = PREFIX.to_owned();
    let mut arguments = Vec::<String>::new();
    if let Some(item) = style {
        for entry in item {
            arguments.push(entry.to_owned());
        }
    }
    if let Some(item) = foreground {
        arguments.push(item.to_owned());
    }
    if let Some(item) = background {
        arguments.push(item.to_owned());
    }

    // panic if no arguments provided since this is a programming mistake
    // which should not
    if arguments.is_empty() {
        panic!("no arguments provided to 'generate_style_sequence()'");
    }
    sequence.push_str(&arguments.join(";"));
    sequence.push_str(TERMINATION); // terminate sequence with the termination character
    sequence
}

/// generates a 256 color sequence
/// see `generate_rgb_sequence(..)` for details
pub fn generate_256color_sequence(context: ColorContext, color: u8) -> String {
    let mut sequence = PREFIX.to_owned();
    // choose context
    match context {
        ColorContext::Foreground => sequence.push_str(COLORS.fg.extended),
        ColorContext::Background => sequence.push_str(COLORS.bg.extended),
    };
    // make it a rgb sequence
    sequence.push_str(";5;");
    sequence.push_str(&format!("{color}m"));
    sequence
}

/// generates a rgb color sequence
/// **note**: not all terminal emulators support rgb colors
///
/// rgb sequences are built the same as 256 color sequences:
/// `\x1B[<context>;2;<r>;<g>;<b>m`
/// where *context* is either '38' or '48' for foreground and background respectively
/// and *r,g,b* are the values of each color channel
pub fn generate_rgb_sequence(context: ColorContext, red: u8, green: u8, blue: u8) -> String {
    let mut sequence = PREFIX.to_owned();
    // choose context
    match context {
        ColorContext::Foreground => sequence.push_str(COLORS.fg.extended),
        ColorContext::Background => sequence.push_str(COLORS.bg.extended),
    };
    // make it a rgb sequence
    sequence.push_str(";2;");
    sequence.push_str(&format!("{red};{green};{blue}m"));
    sequence
}

/// generates a padding string for numbers in a list
pub fn make_padding_string(len: usize) -> String {
    // determine padding needed to align the paths
    String::from_utf8(vec![b' '; len]).unwrap()
}

/// convert color setting to ansi escape sequence
pub fn parse_color(color: String) -> Result<String> {
    // check for numbered color
    if let Ok(numbered) = color.parse::<u8>() { // TODO: only accept numbers between 16 and 256
        return Ok(generate_256color_sequence(
            ColorContext::Foreground,
            numbered,
        ));
    }
    // check for rgb color
    if color.as_bytes()[0] == b'#' && color.len() == 7 {
        // match u8::from_str_radix(&color, 16) {
        let red = match u8::from_str_radix(&color[1..=2], 16) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(format!(
                    "-- failed to parse rgb color `{}` in config file",
                    color
                )))
            }
        };
        let green = match u8::from_str_radix(&color[3..=4], 16) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(format!(
                    "-- failed to parse rgb color `{}` in config file",
                    color
                )))
            }
        };
        let blue = match u8::from_str_radix(&color[5..=6], 16) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(format!(
                    "-- failed to parse rgb color `{}` in config file",
                    color
                )))
            }
        };
        return Ok(generate_rgb_sequence(
            ColorContext::Foreground,
            red,
            green,
            blue,
        ));
    }
    // check for named color
    match color.to_ascii_lowercase().as_str() {
        "black" => Ok(generate_style_sequence(None, Some(COLORS.fg.black), None)),
        "red" => Ok(generate_style_sequence(None, Some(COLORS.fg.red), None)),
        "green" => Ok(generate_style_sequence(None, Some(COLORS.fg.green), None)),
        "yellow" => Ok(generate_style_sequence(None, Some(COLORS.fg.yellow), None)),
        "blue" => Ok(generate_style_sequence(None, Some(COLORS.fg.blue), None)),
        "magenta" => Ok(generate_style_sequence(None, Some(COLORS.fg.magenta), None)),
        "cyan" => Ok(generate_style_sequence(None, Some(COLORS.fg.cyan), None)),
        "white" => Ok(generate_style_sequence(None, Some(COLORS.fg.white), None)),
        "default" => Ok(generate_style_sequence(None, Some(COLORS.fg.default), None)),
        _ => Err(Error::other(format!(
            "-- could not parse color `{}` in config file",
            color
        ))),
    }
}
