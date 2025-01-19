#![allow(dead_code)]

use std::io::{Result, Error};

pub const ESC: &str = "\x1b";
pub const PREFIX: &str = "\x1b[";
pub const RESET_ARG: &str = "0";
pub const TERMINATION: &str = "m";
pub const RESET_SEQ: &str = "\x1b[0m";
pub const FG: ColorContext = ColorContext::Foreground;
pub const BG: ColorContext = ColorContext::Background;

pub const STYLES: Styles = Styles {
    set: StyleCodes {
        bold: "1",
        dim: "2",
        italic: "3",
        underlined: "4",
        blinking: "5",
        reversed: "7",
        invisible: "8",
        strikethrough: "9",
    },
    reset: StyleCodes {
        bold: "22",
        dim: "22",
        italic: "23",
        underlined: "24",
        blinking: "25",
        reversed: "27",
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

#[derive(Debug, Clone)]
pub struct Styles {
    pub set: StyleCodes,
    pub reset: StyleCodes,
}

#[derive(Debug, Clone)]
pub struct StyleCodes {
    pub bold: &'static str,
    pub dim: &'static str,
    pub italic: &'static str,
    pub underlined: &'static str,
    pub blinking: &'static str,
    pub reversed: &'static str,
    pub invisible: &'static str,
    pub strikethrough: &'static str,
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub fg: ColorCodes,
    pub bg: ColorCodes,
}

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
    style: Option<&str>,
    foreground: Option<&str>,
    background: Option<&str>,
) -> String {
    // assemble sequence as vector of string
    // this way the semicolons to separate arguments
    // can be inserted with 'join()'
    let mut sequence = PREFIX.to_owned();
    let mut arguments = Vec::<String>::new();
    if let Some(item) = style {
        arguments.push(item.to_owned());
    }
    if let Some(item) = foreground {
        arguments.push(item.to_owned());
    }
    if let Some(item) = background {
        arguments.push(item.to_owned());
    }

    // panic if no arguments provided since this is a programming mistake
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
    if color < 16 {
        return "".to_owned();
    }
    let mut sequence = PREFIX.to_owned();
    // choose context
    match context {
        ColorContext::Foreground => sequence.push_str(COLORS.fg.extended),
        ColorContext::Background => sequence.push_str(COLORS.bg.extended),
    };
    // make it a rgb sequence
    sequence.push_str(";5;");
    sequence.push_str(&format!("{color}{TERMINATION}"));
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
/// input format is a quoted string (either double or single)
/// the style can be a combination of **one** color and
/// one or more style options (bold, italic, underlined, strikethrough)
pub fn parse_style(arg: &String) -> Result<String> {
    let mut colors: Vec<String> = Vec::<String>::new();
    let mut styles: Vec<String> = Vec::<String>::new();

    // separate style options
    let mut tokens: Vec<String> = arg.split([' ', ',', '\"', '\'']).map(|entry| entry.trim().to_lowercase()).collect();
    tokens.retain(|entry| !entry.is_empty());

    // parse options
    for option in tokens {
        // parse numbered colors
        if let Ok(sequence) = parse_numbered_color(&option) {
            colors.push(sequence);
            continue;
        }

        // parse rgb colors
        if let Ok(sequence) = parse_rgb_color(&option) {
            colors.push(sequence);
            continue;
        }

        // parse styles and named colors
        match option.as_str() {
            // styles
            "bold" => styles.push(generate_style_sequence(Some(STYLES.set.bold), None, None)),
            "dim" => styles.push(generate_style_sequence(Some(STYLES.set.dim), None, None)),
            "italic" => styles.push(generate_style_sequence(Some(STYLES.set.italic), None, None)),
            "underlined" => styles.push(generate_style_sequence(Some(STYLES.set.underlined), None, None)),
            "blinking" => styles.push(generate_style_sequence(Some(STYLES.set.blinking), None, None)),
            "reversed" => styles.push(generate_style_sequence(Some(STYLES.set.reversed), None, None)),
            "invisible" => styles.push(generate_style_sequence(Some(STYLES.set.invisible), None, None)),
            "strikethrough" => styles.push(generate_style_sequence(Some(STYLES.set.strikethrough), None, None)),
            // named colors
            "black" => colors.push(generate_style_sequence(None, Some(COLORS.fg.black), None)),
            "red" => colors.push(generate_style_sequence(None, Some(COLORS.fg.red), None)),
            "green" => colors.push(generate_style_sequence(None, Some(COLORS.fg.green), None)),
            "yellow" => colors.push(generate_style_sequence(None, Some(COLORS.fg.yellow), None)),
            "blue" => colors.push(generate_style_sequence(None, Some(COLORS.fg.blue), None)),
            "magenta" => colors.push(generate_style_sequence(None, Some(COLORS.fg.magenta), None)),
            "cyan" => colors.push(generate_style_sequence(None, Some(COLORS.fg.cyan), None)),
            "white" => colors.push(generate_style_sequence(None, Some(COLORS.fg.white), None)),
            "default" => colors.push(generate_style_sequence(None, Some(COLORS.fg.default), None)),
            _ => return Err(Error::other(format!("-- could not parse style token `{}` in config file", option))),
        };

    };

    if colors.len() > 1 {
        return Err(Error::other(format!("-- too many colors found in setting <{}>", arg)));
    }
    if !colors.is_empty() {
        styles.push(colors.pop().unwrap());
    }
    Ok(styles.join(""))
}

fn parse_numbered_color(string: &String) -> Result<String> {
    // check for numbered color
    if let Ok(number) = string.parse::<u8>() {
        return Ok(generate_256color_sequence(
            ColorContext::Foreground,
            number,
        ));
    }
    Err(Error::other(format!("no numbered color found in '{}'", string)))
}

fn parse_rgb_color(string: &String) -> Result<String> {
    // check for rgb color
    if string.as_bytes()[0] == b'#' && string.len() == 7 {
        let red = match u8::from_str_radix(&string[1..=2], 16) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(format!(
                    "-- failed to parse rgb color `{}` in config file",
                    string
                )))
            }
        };
        let green = match u8::from_str_radix(&string[3..=4], 16) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(format!(
                    "-- failed to parse rgb color `{}` in config file",
                    string
                )))
            }
        };
        let blue = match u8::from_str_radix(&string[5..=6], 16) {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(format!(
                    "-- failed to parse rgb color `{}` in config file",
                    string
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
    Err(Error::other(format!("no rgb color found in '{}'", string)))
}
