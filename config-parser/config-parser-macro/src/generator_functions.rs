use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Field, Meta, Path, punctuated::Punctuated, token::Comma};
use quote::quote;

pub fn gen_parse_from_string(config_name: &Ident, assignments: &TokenStream) -> TokenStream {
    quote! {
        /// tries to parse config from a string
        /// if **convert_styles** == true, the settings marked with
        /// `style_config` are converted to ansi escape sequences to
        /// style terminal ouput
        pub fn parse_from_string(&mut self, input: &String) -> std::io::Result<()> {
            let mut #config_name : std::collections::HashMap<String, String> = parse_config_file(input)?;

            #assignments

            if !#config_name.is_empty() {
                let leftovers = #config_name.keys().cloned().collect::<Vec<String>>();
                return Err(std::io::Error::other(format!("the following settings were not recognised: {:#?}", leftovers)));
            }
            Ok(())
        }
    }
}

pub fn gen_parse_from_map(config_name: &Ident, assignments: &TokenStream) -> TokenStream {
    quote! {
        /// **do not call**
        /// this function needs to be public for nested configs but is not intended
        /// to be called by the user
        pub fn parse_from_map(&mut self, input: &mut std::collections::HashMap<String, String>) -> std::io::Result<()> {
            let mut #config_name = input;

            #assignments

            Ok(())
        }
    }
}

pub fn gen_to_ansi_sequences(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut conversions: TokenStream = TokenStream::new();
    'fields: for field in fields.iter() {
        let attr = &field.attrs;
        let name = match &field.ident {
            Some(value) => value,
            // skip anonymous fields
            None => continue 'fields,
        };
        for attribute in attr {
            if let Attribute{ meta: Meta::Path( Path{segments: attr_name, ..} ), .. } = attribute {
                match attr_name.first() {
                    Some(value) => if value.ident == "style_config" {
                        conversions.extend(quote! {
                            self.#name = match config_parser::parse_style(&self.#name) {
                                Ok(value) => value,
                                Err(_) => return Err(std::io::Error::other(format!("failed to convert '{}' to ansi escape sequence", self.#name))),
                            };
                        });
                    } else if value.ident == "nested_config" {
                        conversions.extend(quote! {
                            self.#name.to_ansi_sequences()?;
                        });
                    },
                    None => (),
                }
            }
        }
    };
    quote!{
        fn to_ansi_sequences(&mut self) -> std::io::Result<()> {
            #conversions
            Ok(())
        }
    }
}

pub fn gen_to_string(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    quote! {
        /// prints configuration to `String`
        //pub fn to_string() -> String {
        //    let mut default = "# default configuration file for `navigate`\n".to_string();
        //}
    }
}

pub fn gen_config_assignments(fields: &Punctuated<Field, Comma>, config_map_name: &syn::Ident) -> TokenStream {
    let mut assignments : TokenStream = TokenStream::new();
    'fields: for field in fields.iter() {
        let attr = &field.attrs;
        let name = match &field.ident {
            Some(value) => value,
            // skip anonymous fields
            None => continue 'fields,
        };
        let name_string: String = name.to_string();
        let ty = &field.ty;
        for attribute in attr {
            if let Attribute{ meta: Meta::Path( Path{segments: attr_name, ..} ), .. } = attribute {
                match attr_name.first() {
                    Some(value) => if value.ident == "nested_config" {
                        assignments.extend(quote! {
                            self.#name.parse_from_map(&mut #config_map_name);
                        });
                        continue 'fields;
                    } else if value.ident == "no_config" {
                        continue 'fields;
                    },
                    None => (),
                }
            }
        }
        assignments.extend(quote! {
            self.#name = match #config_map_name.get(#name_string) {
                Some(value) => match value.parse::<#ty>() {
                    Ok(parsed) => {
                        parsed
                    },
                    Err(_) => {
                        self.#name.clone()
                    },
                },
                None => self.#name.clone(),
            };
        });
    }
    assignments
}

