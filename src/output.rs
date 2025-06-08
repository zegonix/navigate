#![allow(unused)]

use clap::builder::EnumValueParser;

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
    pub fn print_output(&self, config: Option<&Config>) {
        let default = Config::default();
        let config = if let Some(value) = config {
            value
        } else {
            &default
        };
        let mut info: String = self.info.iter().map(|entry| format!("echo '{}'", entry)).collect::<Vec<String>>().join(" && ");
        let mut warning: String = self.warning.iter().map(|entry| format!("echo '{}'", entry)).collect::<Vec<String>>().join(" && ");
        let mut error: String = self.error.iter().map(|entry| format!("echo '{}'", entry)).collect::<Vec<String>>().join(" && ");
        let mut command: String = self.command.join(" && ");
        let mut output: Vec<String> = Vec::<String>::new();

        if !info.is_empty() {
            output.push(info);
        }
        if !warning.is_empty() {
            warning = format!("echo '{}' && {}", config.styles.warning_style, warning);
            output.push(warning);
        }
        if !error.is_empty() {
            error = format!("echo '{}' && {}", config.styles.error_style, error);
            output.push(error);
        }
        if !command.is_empty() {
            output.push(command);
        }

        println!("{}", output.join(" && echo && "));
    }
}
