//! implements a struct and methods for bookmarks

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;
use dirs::{config_dir, home_dir};

use super::{config::*, util::to_rooted};
use config_parser::{apply_format, make_padding_string};

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

            // if !path.is_dir() {
            //     continue;
            // }

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
        if self.bookmarks.contains_key(name) {
            return Err(Error::other(format!("-- bookmark with name `{name}` already exists")));
        }
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

    /// 
    pub fn remove_invalid_paths(&mut self) -> Result<()> {
        self.bookmarks.retain(|_, path| path.is_dir());
        self.write_bookmark_file()?;

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
            for (raw_name, raw_path) in &self.bookmarks {
                let padding: String = make_padding_string(max_name_len - raw_name.len());
                let mut name: String = raw_name.clone();
                let mut separator: String = config.format.bookmarks_separator.clone();
                let mut path: String = raw_path.clone().into_os_string().into_string().unwrap();

                if config.format.show_home_as_tilde {
                    let home: String = match home_dir() {
                        Some(value) => match value.into_os_string().into_string() {
                            Ok(value) => value,
                            Err(error) => return Err(Error::other(format!("-- failed to conver home directory to string: {}", error.to_str().unwrap()))),
                        },
                        None => return Err(Error::other("-- `bookmarks_home_as_tilde` = true, but home directory can't be determined")),
                    };
                    path = path.replace(&home, "~");
                }

                if raw_path.is_dir() {
                    let slash: String = apply_format(&"/".to_owned(), &config.styles.bookmarks_punct_style)?;
                    let mut segments: Vec<String> = path.split('/').map(|element| element.to_owned()).collect();
                    for element in segments.iter_mut() {
                        *element = apply_format(&element, &config.styles.bookmarks_path_style)?;
                    }
                    path = segments.join(&slash);

                    name = apply_format(&name, &config.styles.bookmarks_name_style)?;
                    separator = apply_format(&separator, &config.styles.bookmarks_seperator_style)?;
                }

                let mut line: String;
                if config.format.align_separators {
                    line = format!("{}{}{}{}\n", name, padding, separator, path);
                } else {
                    line = format!("{}{}{}{}\n", name, separator, padding, path);
                }
                if !raw_path.is_dir() {
                    line = apply_format(&line, &config.styles.bookmarks_invalid_style)?;
                }

                buffer.push_str(&line);
            }
        }
        Ok(buffer)
    }

    /// get bookmarknames as space separated values in one string (for shell completions)
    pub fn get_bookmark_names(&self) -> String {
        let mut copy = self.bookmarks.clone();

        for (name, path) in copy.clone() {
            if !path.is_dir() {
                _ = copy.remove(&name);
            }
        }
        let names: Vec<String> = copy.keys().cloned().collect();
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
