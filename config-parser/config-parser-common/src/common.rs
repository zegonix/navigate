#![allow(dead_code)]

pub fn parse_config_file(input: &String) -> std::io::Result<std::collections::HashMap<String, String>> {
    let mut config = std::collections::HashMap::<String, String>::new();
    let lines = input.lines();

    for line in lines {
        let line = line.trim();
        // ignore empty lines
        if line.is_empty() { continue; }

        if line.starts_with("[") {
            // check for table
            if !line.ends_with("]") {
                // TODO: implement error handling
            } else if line.contains(' ') {
                // TODO: implement error handling
            }
            //let tokens = line.split('.');
            // TODO: implement hirarchical map
        } else {
            // check for config
            let mut tokens: Vec<&str> = line.split('=').map(|entry| entry.trim()).collect();
            tokens.retain(|entry| !entry.is_empty());
            // check for valid input
            if tokens.len() != 2 {
                // println!("error in line'", line);
                continue;
            }
            //// clean up value: remove quotes.. TODO:
            //let mut options: Vec<&str> = tokens[1].split(['\'', '\"']).collect::<Vec<&str>>().join(',');

            config.insert(tokens[0].to_string(), tokens[1].to_string());
        }

    }
    Ok(config)
}

