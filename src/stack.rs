#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use sysinfo::{Pid, System};

use crate::make_padding_string;

use super::{apply_format, config::*};

#[derive(Debug, Clone)]
pub struct Stack {
    pid: u32,
    path: PathBuf,
    stack: Vec<PathBuf>,
}

impl Stack {
    pub fn new(process_id: u32) -> Result<Self> {
        let mut stack: Stack = Stack {
            pid: process_id,
            path: PathBuf::new(),
            stack: Vec::<PathBuf>::new(),
        };
        stack.build_stack()?;

        Ok(stack)
    }

    /// formats and prints stack to string
    pub fn to_formatted_string(&self, config: &Settings) -> Result<String> {
        let mut buffer: String = "".to_string();

        if self.stack.is_empty() {
            buffer.push_str("-- the stack is empty");
        } else {
            // print stack to string
            let max_num_len = self.stack.len().to_string().len();
            for (n, item) in self.stack.iter().rev().enumerate() {
                let padding = make_padding_string(max_num_len - n.to_string().len());
                let number = apply_format(&n.to_string(), &config.styles.stack_number_style);
                let separator = apply_format(
                    &config.format.stack_separator,
                    &config.styles.stack_separator_style,
                );
                let path = apply_format(item.to_str().unwrap(), &config.styles.stack_path_style);
                if config.format.align_separators {
                    buffer.push_str(&format!("{}{}{}{}\n", number, padding, separator, path));
                } else {
                    buffer.push_str(&format!("{}{}{}{}\n", number, separator, padding, path));
                }
            }
        }
        Ok(buffer)
    }

    /// clear stack by deleting the associated stack file
    pub fn clear_stack(&mut self, config: &Settings) -> Result<()> {
        fs::remove_file(self.path.clone())?;
        print!("echo 'stack cleared successfully.'");
        Ok(())
    }

    /// push entry to stack
    /// returns updated stack
    pub fn push_entry(&mut self, path: &Path) -> Result<&Vec<PathBuf>> {
        let abs_path = path.canonicalize()?;
        self.stack.push(abs_path);
        self.write_stack_file()?;
        Ok(&self.stack)
    }

    /// pop entry from stack
    /// return popped entry
    pub fn pop_entry(&mut self, num_entries: Option<usize>) -> Result<PathBuf> {
        let mut num = num_entries.unwrap_or(1);
        if num == 0 || num > self.stack.len() {
            num = self.stack.len();
        }
        let mut dropped_entries = self.stack.drain((self.stack.len() - num)..);
        let entry = dropped_entries.nth(0);
        drop(dropped_entries);
        self.write_stack_file()?;
        match entry {
            Some(entry) => Ok(entry),
            None => Err(Error::other(
                "-- pop failed to retrieve item from stack, it might be empty",
            )),
        }
    }

    /// get entry by number without removing it from the stack
    /// return nth last entry
    pub fn get_entry_by_number(&mut self, entry_number: usize) -> Result<&PathBuf> {
        // index from the end of the vector as new entries are appended at the end of the list
        let index = match self.stack.len().checked_sub(entry_number) {
            Some(value) => value,
            None => return Err(Error::other("-- no entry found at request index")),
        };
        match self.stack.get(index) {
            Some(item) => Ok(item),
            None => Err(Error::other("-- failed to retrieve stack entry by number")),
        }
    }

    /// clean up dead stack files, parse and build stack
    fn build_stack(&mut self) -> Result<()> {
        let stack_dir: PathBuf = match PathBuf::from_str("/tmp/navigation/") {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::other(
                    "-- failed to create path object of the stack directory",
                ))
            }
        };
        let mut sys = System::new_all();
        sys.refresh_all();
        let procs = sys.processes();

        if stack_dir.is_dir() {
            // clean up stack files of expired processes
            let members = fs::read_dir(stack_dir.clone())?;
            for entry in members {
                let entry = entry?;
                let process_id = match Pid::from_str(match entry.file_name().to_str() {
                    Some(value) => value,
                    None => return Err(Error::other("-- failed to convert file name to str")),
                }) {
                    Ok(value) => value,
                    Err(error) => return Err(Error::other(error.to_string())),
                };
                if !procs.contains_key(&process_id) {
                    match fs::remove_file(entry.path()) {
                        Ok(value) => value,
                        Err(error) => return Err(Error::other(error.to_string())),
                    }
                }
            }
        } else {
            // create temporary directory to store the stack
            fs::create_dir(stack_dir.clone())?;
        }

        self.path = stack_dir.clone();
        self.path.push(PathBuf::from(&self.pid.to_string()));
        if self.path.is_file() {
            // read and parse stack file
            self.read_stack_file(&self.path.clone())?;
        } else {
            // create stack file and store current path
            File::create(self.path.clone())?;
        }
        self.cleanup_stack();

        Ok(())
    }

    /// parse stack file
    fn read_stack_file(&mut self, stack_file_path: &PathBuf) -> Result<()> {
        let stack = fs::read_to_string(stack_file_path)?;
        let stack = stack.split("\n");
        for entry in stack {
            self.stack.push(PathBuf::from(entry));
        }

        Ok(())
    }

    /// remove invalid paths from stack
    fn cleanup_stack(&mut self) {
        if !self.stack.is_empty() {
            self.stack.retain(|entry| entry.is_dir());
        }
    }

    /// write stack current stack to file to save it for next execution
    fn write_stack_file(&mut self) -> Result<()> {
        let mut output = Vec::<&str>::new();
        for entry in &self.stack {
            output.push(match entry.to_str() {
                Some(value) => value,
                None => return Err(Error::other("-- failed to convert stack entry to string")),
            });
        }
        fs::write(self.path.clone(), output.join("\n"))?;

        Ok(())
    }
}
