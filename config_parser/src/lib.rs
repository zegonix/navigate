use common::gen_config_load_function;
use proc_macro2::{TokenStream}; // TODO: change to proc_macro2, to hopefully fix #assignments
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Ident};
use quote::{quote};

#[macro_use]
mod common;

/// **for structs only**
/// - implements `parse_config(&mut self, input: &String) -> Result<()>`
/// which parses a string and fills the fills recognised values into the struct
/// - implements `write_default_config() -> Result<String>`
/// which write a default configuration, in case the documentation is lacking
#[proc_macro_derive(ConfigParser, attributes(nested_config))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let config_name = syn::Ident::new("config", name.span()); // TODO: use correct span
    let fields = if let syn::Data::Struct(syn::DataStruct{ fields: syn::Fields::Named(syn::FieldsNamed{ ref named, .. }), .. }) = ast.data {
        named
    } else {
        todo!("use `syn::Error` to return comprehensive error message");
    };
    let assignments = match gen_config_load_function(fields, &config_name) {
        Ok(value) => value,
        Err(_) => panic!("loading config failed"),
    };

    //let string = format!("{:#?}", ast);
    //_ = std::fs::write("test.txt", string);

    let expanded_stream: TokenStream = quote::quote! {
        impl #name {
            pub fn parse_from_string(&mut self, input: &String) -> std::io::Result<()> {
                let #config_name : std::collections::HashMap<String, String> = Self::parse_config_file(input)?;

                #assignments

                Ok(())
            }

            /// **do not call**
            /// this function needs to be public for nested configs but is not intended
            /// to be called by the user
            pub fn parse_from_map(&mut self, input: &std::collections::HashMap<String, String>) -> std::io::Result<()> {
                let #config_name = input;

                #assignments

                Ok(())
            }

            fn parse_config_file(input: &String) -> std::io::Result<std::collections::HashMap<String, String>> {
                let mut config = std::collections::HashMap::<String, String>::new();
                let lines = input.lines();

                for line in lines {
                    let mut line = line.trim();
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
                        if tokens.len() != 2 {
                            // println!("error in line'", line);
                            continue;
                        }
                        config.insert(tokens[0].to_string(), tokens[1].to_string());
                    }

                }
                Ok(config)
            }

            // TODO: implement
            pub fn write_default_config(&self) -> Result<()> {
                Ok(())
            }

            // TODO: implement function to parse style settings
        }
    }.into();
    expanded_stream.into()
    //TokenStream::new()
}

