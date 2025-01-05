#![allow(dead_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote};

use syn::{Attribute, Error, Field, Meta, Path, punctuated::Punctuated, token::Comma};

pub fn gen_config_load_function(fields: &Punctuated<Field, Comma>, config_map_name: &String) -> Result<TokenStream, Error> {
    let assignments = fields.iter().map(|f|{
        let attr = &f.attrs;
        let name = match &f.ident {
            Some(value) => value,
            // skip anonymous fields
            None => return quote! {},
        };
        let name_string: String = name.to_string();
        let ty = &f.ty;
        for attribute in attr {
            if let Attribute{ meta: Meta::Path( Path{segments: attr_name, ..} ), .. } = attribute {
                match attr_name.first() {
                    Some(value) => if value.ident == "nested_config" {
                        return quote! {#name.parse_from_map(&#config_map_name)}
                    },
                    None => (),
                }
            }
        }
        quote! {
            self.#name = match #config_map_name.get(#name_string) {
                Some(value) => value,
                None => self.#name,
            }
        }
    });
    Ok(quote! {assignments}.into())
}

//pub fn parse_config_file(input: &String) -> std::io::Result<std::collections::HashMap<String, String>> {
//    let mut config = std::collections::HashMap::<String, String>::new();
//    let lines = input.lines();
//
//    for line in lines {
//        let line = line.trim();
//        // ignore empty lines
//        if line.is_empty() { continue; }
//
//        if line.starts_with("[") {
//            // check for table
//            if !line.ends_with("]") {
//                // TODO: implement error handling
//            } else if line.contains(' ') {
//                // TODO: implement error handling
//            }
//            //let tokens = line.split('.');
//            // TODO: implement hirarchical map
//        } else {
//            // check for config
//            let mut tokens: Vec<&str> = line.split('=').map(|entry| entry.trim()).collect();
//            tokens.retain(|entry| !entry.is_empty());
//            if tokens.len() != 2 {
//                // println!("error in line'", line);
//                continue;
//            }
//            config.insert(tokens[0].to_string(), tokens[1].to_string());
//        }
//    }
//    Ok(config)
//}
//
