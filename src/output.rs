#![allow(unused)]

use clap::builder::EnumValueParser;
use config_parser::{apply_format, parse_ansi_set, parse_ansi_unset};

use super::config::*;

use std::backtrace::Backtrace;

/// takes strings to output because the application is
/// called by a script which executes the output with `eval`
pub struct Output {
    /// takes direct command output with custom formatting
    /// **NOTE** - strings in `command` are not wrapped
    /// with `echo` or similar and thus are interpretted as
    /// commands
    command: Vec<String>,
    /// takes formatted output to be printed with `echo`
    /// strings in `info` are prepended with `echo` but
    /// do not get any formatting applied
    info: Vec<String>,
    /// takes warnings about command input or the state of
    /// application
    warning: Vec<String>,
    /// takes error messages about command input or the
    /// state of application
    error: Vec<String>,
}

impl Output {
    pub fn new() -> Self {
        Self {
            command: Vec::<String>::new(),
            info: Vec::<String>::new(),
            warning: Vec::<String>::new(),
            error: Vec::<String>::new(),
        }
    }

    /// push a command to the output pipeline
    pub fn push_command(&mut self, command: &String) {
        self.command.push(command.to_string());
    }

    /// push an information to the output pipeline
    pub fn push_info(&mut self, info: &String) {
        self.info.push(info.to_string());
    }

    /// push a warning to the output pipeline
    pub fn push_warning(&mut self, warning: &String) {
        self.warning.push(warning.to_string());
    }

    /// push an error to the output pipeline
    pub fn push_error(&mut self, error: &String) {
        self.error.push(error.to_string());
    }

    /// format and print styled output
    /// NOTE - this will execute any commands held by `command`
    pub fn print_output(&mut self, config: Option<&Config>) {
        let default = Config::default();
        let config = if let Some(value) = config {
            value
        } else {
            &default
        };

        if !self.error.is_empty() {
            self.push_command(&"false".to_owned());
        }

        let mut output: Vec<String> = Vec::<String>::new();

        if !self.info.is_empty() {
            let mut info: String = self.info.iter().map(|entry| format!("echo '{}'", entry)).collect::<Vec<String>>().join(" && ");
            output.push(info);
        }
        if !self.warning.is_empty() {
        let mut warning: String = self.warning.iter().map(|entry| apply_format(entry, &config.styles.warning_style).unwrap()).map(|entry| format!("echo '{}'", entry)).collect::<Vec<String>>().join(" && ");
            output.push(warning);
        }
        if !self.error.is_empty() {
            let mut error: String = self.error.iter().map(|entry| apply_format(entry, &config.styles.error_style).unwrap()).map(|entry| format!("echo '{}'", entry)).collect::<Vec<String>>().join(" && ");
            output.push(error);
        }
        if !self.command.is_empty() {
            let mut command: String = self.command.join(" && ");
            output.push(command);
        }

        println!("{}", output.join(" && "));
    }
}
