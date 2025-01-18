#![allow(dead_code)]

use std::collections::HashMap;

/// Holds config, value is either a `String` or a nested `ConfigMap`
pub type ConfigMap = HashMap<String, ConfigElement>;

/// Element of Config, either a `String` for a setting
/// or a nested `ConfigMap`
pub enum ConfigElement {
    Setting(String),
    Nested(ConfigMap),
}

pub fn parse_config_file(input: &String) -> (ConfigMap, Vec<String>) {
    let mut config = ConfigMap::new();
    let mut pointer: &mut ConfigMap = &mut config;
    let mut messages: Vec<String> = Vec::<String>::new();
    let lines = input.lines();

    for (n, line) in lines.enumerate() {
        let line = line.trim();
        // ignore empty lines
        if line.is_empty() { continue; }

        if line.starts_with("[[") {
            messages.push(format!("error on line #{n} - arrays are not supported in config file:\n{}", line));
        } else if line.starts_with("[") {
            // check for table
            if !line.ends_with("]") {
                messages.push(format!("error on line #{n} - table name is not properly terminated (missing ']'):\n{}", line));
            } else if line.contains([' ', '\t']) {
                messages.push(format!("error on line #{n} - no white space allowed in table names"));
            }
            let mut tokens: Vec<&str> = line.split(['.', '[', ']']).map(|entry| entry.trim()).collect();
            tokens.retain(|entry| !entry.is_empty());

            pointer = &mut config;
            for token in tokens {
                pointer = match pointer.entry(token.to_string()).or_insert(ConfigElement::Nested(ConfigMap::new())) {
                    ConfigElement::Nested(entry) => entry,
                    _ => {
                        panic!("error occured handling line #{} - tried to insert a `Nested` element into ConfigMap, but found a `Setting` element of the same name ({})", n, token);
                    }
                };
            }
        } else {
            // check for config
            let mut tokens: Vec<&str> = line.split('=').map(|entry| entry.trim()).collect();
            tokens.retain(|entry| !entry.is_empty());
            // check for valid input
            if tokens.len() != 2 {
                // println!("error in line'", line);
                continue;
            }
            // remove characters not wanted in output
            let key = tokens[0];
            let value = tokens[1].replace(&['\"', '\''][..], "");

            pointer.insert(key.to_string(), ConfigElement::Setting(value));
        }

    }
    (config, messages)
}


