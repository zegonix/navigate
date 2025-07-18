#![allow(dead_code)]

use std::{fs, usize};
use std::fs::File;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use sysinfo::{Pid, System};
use dirs::home_dir;

use crate::make_padding_string;
use super::{apply_format, config::*, util::to_rooted};

#[derive(Debug, Clone)]
pub struct Stack {
    pid: u32,
    path: PathBuf,
    stack: Vec<PathBuf>,
}

impl Stack {
    const STACK_FILE_DIRECTORY: &str = "/tmp/navigate/";

    pub fn new(config: &Config, process_id: u32) -> Result<Self> {
        let mut stack: Stack = Stack {
            pid: process_id,
            path: PathBuf::new(),
            stack: Vec::<PathBuf>::new(),
        };
        stack.build_stack(config)?;

        Ok(stack)
    }

    /// formats and prints stack to string
    pub fn to_formatted_string(&self, config: &Config) -> Result<String> {
        let mut buffer: String = "".to_string();

        if self.stack.is_empty() {
            buffer.push_str("-- the stack is empty");
        } else {
            // print stack to string
            let max_num_len: usize = self.stack.len().to_string().len();
            for (n, item) in self.stack.iter().rev().enumerate() {
                let padding: String = make_padding_string(max_num_len - n.to_string().len());
                let mut number: String = n.to_string();
                let mut separator: String = config.format.stack_separator.clone();
                let mut path: String = item.clone().into_os_string().into_string().unwrap();

                if config.format.show_home_as_tilde {
                    let home: String = match home_dir() {
                        Some(value) => match value.into_os_string().into_string() {
                            Ok(value) => value,
                            Err(error) => return Err(Error::other(format!("-- failed to conver home directory to string: {}", error.to_str().unwrap()))),
                        },
                        None => return Err(Error::other("-- `stack_home_as_tilde` = true, but home directory can't be determined")),
                    };
                    path = path.replace(&home, "~");
                }

                if item.is_dir() {
                    let slash: String = apply_format(&"/".to_owned(), &config.styles.stack_punct_style)?;
                    let mut segments: Vec<String> = path.split('/').map(|element| element.to_owned()).collect();
                    for element in segments.iter_mut() {
                        *element = apply_format(&element, &config.styles.stack_path_style)?;
                    }
                    path = segments.join(&slash);

                    number = apply_format(&number, &config.styles.stack_number_style)?;
                    separator = apply_format(&separator, &config.styles.stack_separator_style)?;
                }

                let mut line: String;
                if config.format.stack_hide_numbers {
                    line = format!("{}\n", path);
                } else if config.format.align_separators {
                    line = format!("{}{}{}{}\n", number, padding, separator, path);
                } else {
                    line = format!("{}{}{}{}\n", number, separator, padding, path);
                }
                if !item.is_dir() {
                    line = apply_format(&line, &config.styles.stack_invalid_style)?;
                }

                buffer.push_str(&line);
            }
        }
        Ok(buffer)
    }

    /// clear stack by deleting the associated stack file
    pub fn clear_stack(&mut self) -> Result<()> {
        fs::remove_file(self.path.clone())?;
        Ok(())
    }

    /// push entry to stack
    /// returns updated stack
    pub fn push_entry(&mut self, path: &Path) -> Result<&Vec<PathBuf>> {
        let mut path: PathBuf = path.to_path_buf();
        to_rooted(&mut path)?;

        // append path to stack and write stack file to save changes
        self.stack.push(path);
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
        if entry_number >= self.stack.len() {
            return Err(Error::other(format!("-- requested item ({entry_number}) out of bounds (stack.len() = {})", self.stack.len())));
        }
        match self.stack.iter().rev().nth(entry_number) {
            Some(value) => Ok(value),
            None => return Err(Error::other("-- failed to retrieve stack element #{entry_number}")),
        }
    }

    /// rotate stack so that the <entry_number> is the latest
    /// entry (first to be popped)
    pub fn rotate_stack(&mut self, entry_number: usize) -> Result<()> {
        if 0 == entry_number {
            return Ok(());
        } else if self.stack.len() <= entry_number {
            return Err(Error::other("-- number to rotate is greater than the stacks length"));
        }

        let mut rotated_stack: Vec<PathBuf> = self.stack.drain(self.stack.len() - entry_number..).collect();

        rotated_stack.extend(self.stack.drain(..));
        self.stack = rotated_stack;

        Ok(())
    }

    /// clean up dead stack files, parse and build stack
    fn build_stack(&mut self, config: &Config) -> Result<()> {
        let stack_dir: PathBuf = match PathBuf::from_str(Self::STACK_FILE_DIRECTORY) {
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

        if config.general.dedup_stack {
            self.dedup_stack();
        }

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

    /// keep only the newest occurence of a path
    fn dedup_stack(&mut self) {
        let mut deduped_stack: Vec<PathBuf> = Vec::new();
        while !self.stack.is_empty() {
            let entry: PathBuf = self.stack.remove(0);
            if !self.stack.contains(&entry) {
                deduped_stack.push(entry);
            }
        }

        self.stack = deduped_stack;
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
