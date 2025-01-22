//! implements a struct and methods for bookmarks

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;
use dirs::config_dir;

use super::{config::*, util::to_rooted};
use config_parser::{apply_format, make_padding_string, RESET_SEQ};

#[derive(Debug, Clone)]
pub struct Bookmarks {
    bookmarks: BTreeMap<String, PathBuf>,
}

impl Bookmarks {
    const BOOKMARK_FILE_PATH: &str = "navigate/bookmarks";

    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut bookmarks = Bookmarks {
            bookmarks: BTreeMap::<String, PathBuf>::new(),
        };
        // get home directory path
        let mut bookmark_file = match config_dir() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to find configuration directory")),
        };
        // expand home directory path to get configuration directory path
        bookmark_file.push(Self::BOOKMARK_FILE_PATH);

        // check if bookmarks file exists, if not create it
        if !bookmark_file.is_file() {
            _ = File::create(bookmark_file.clone())?;
        }

        let bookmarks_str = fs::read_to_string(bookmark_file)?;
        for entry in bookmarks_str.lines() {
            let tokens: Vec<&str> = entry.split("=").collect();
            if tokens.len() != 2 {
                continue;
            }
            let key: String = String::from(tokens[0]);
            let mut path: PathBuf = match PathBuf::from_str(tokens[1]) {
                Ok(value) => value,
                Err(err) => return Err(Error::other(err.to_string())),
            };
            to_rooted(&mut path)?;
            if !path.is_dir() {
                continue;
            }
            bookmarks.bookmarks.insert(key, path);
        }
        Ok(bookmarks)
    }

    /// returns path of bookmark if it exists
    pub fn get_path_by_name(&mut self, name: &str) -> Result<PathBuf> {
        match self.bookmarks.get(name) {
            Some(value) => Ok(value.to_owned()),
            None => Err(Error::other(format!(
                "-- bookmark with name `{}` does not exist",
                name
            ))),
        }
    }

    /// adds a key/value pair to bookmarks and writes the bookmarks file
    pub fn add_bookmark(&mut self, name: &String, path: &PathBuf) -> Result<()> {
        let mut path = path.to_path_buf();
        to_rooted(&mut path)?;
        if !path.is_dir() {
            return Err(Error::other(
                "-- provided path argument does not point to a valid directory",
            ));
        } else {
            self.bookmarks.insert(name.to_string(), path.to_path_buf());
            self.write_bookmark_file()?;
        }
        Ok(())
    }

    /// removes a the entry with key=name if it exists, then writes the bookmarks file
    pub fn remove_bookmark(&mut self, name: &String) -> Result<()> {
        if self.bookmarks.contains_key(name) {
            _ = self.bookmarks.remove(name);
            self.write_bookmark_file()?;
        } else {
            return Err(Error::other(
                "-- bookmark requested to delete does not exist",
            ));
        }
        Ok(())
    }

    /// formats and prints bookmarks to string
    pub fn to_formatted_string(&self, config: &Config) -> Result<String> {
        let mut buffer = String::new();

        if self.bookmarks.is_empty() {
            buffer.push_str("-- there are no bookmarks defined");
        } else {
            let max_name_len =  match self.bookmarks.keys().map(String::len).max() {
                Some(value) => value,
                None => return Err(Error::other("-- failed to determine maximum bookmark name length")),
            };
            for (mark, path) in &self.bookmarks {
                let padding = make_padding_string(max_name_len - mark.len());
                let name = apply_format(mark, &config.styles.bookmarks_name_style);
                let separator = apply_format(
                    &config.format.bookmarks_separator,
                    &config.styles.bookmarks_seperator_style,
                );

                let mut path = apply_format(path.to_str().unwrap(), &config.styles.bookmarks_path_style);
                path = path.replace('/', &format!("{}{}/{}{}", RESET_SEQ, config.styles.bookmarks_punct_style, RESET_SEQ, &config.styles.bookmarks_path_style));

                if config.format.align_separators {
                    buffer.push_str(&format!("{}{}{}{}\n", name, padding, separator, path));
                } else {
                    buffer.push_str(&format!("{}{}{}{}\n", name, separator, padding, path));
                }
            }
        }
        Ok(buffer)
    }

    /// get bookmarknames as space separated values in one string (for shell completions)
    pub fn get_bookmarknames(&self) -> String {
        let mut names: Vec<String> = self.bookmarks.keys().cloned().collect();
        names.join(" ")
    }

    /// writes the bookmarks file
    fn write_bookmark_file(&self) -> Result<()> {
        let mut file_content = String::new();
        for (mark, path) in self.bookmarks.iter() {
            file_content.push_str(&format!("{}={}\n", mark, path.to_str().unwrap()));
        }

        let path = match config_dir() {
            Some(mut value) => {
                value.push(Self::BOOKMARK_FILE_PATH);
                value
            }
            None => return Err(Error::other("-- failed to find configuration directory")),
        };

        fs::write(path, file_content)?;
        Ok(())
    }
}
