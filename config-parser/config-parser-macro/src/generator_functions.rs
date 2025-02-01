use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Expr, ExprLit, Field, Meta, MetaList, MetaNameValue, Path};
use quote::quote;

pub fn gen_parse_from_string(config_name: &Ident, output_name: &Ident, assignments: &TokenStream) -> TokenStream {
    quote! {
        /// tries to parse config from a string
        /// if **convert_styles** == true, the settings marked with
        /// `style_config` are converted to ansi escape sequences to
        /// style terminal ouput
        pub fn parse_from_string(&mut self, input: &String) -> std::io::Result<()> {
            let (mut #config_name, mut #output_name) : (ConfigMap, Vec<String>) = parse_config_file(input);

            #assignments

            if !#config_name.is_empty() {
                let leftovers = #config_name.keys().cloned().collect::<Vec<String>>();
                #output_name.push(format!("the following settings were not recognised: {:#?}", leftovers));
            }
            if !#output_name.is_empty() {
                return Err(std::io::Error::other(format!("{}", #output_name.join("\n"))));
            }
            Ok(())
        }
    }
}

pub fn gen_parse_from_map(config_name: &Ident, output_name: &Ident, assignments: &TokenStream) -> TokenStream {
    quote! {
        /// **do not call**
        /// this function needs to be public for nested configs but is not intended
        /// to be called by the user
        pub fn parse_from_map(&mut self, input: ConfigMap) -> std::io::Result<()> {
            let mut #config_name: ConfigMap = input;
            let mut #output_name: Vec<String> = Vec::<String>::new();

            #assignments

            if !#config_name.is_empty() {
                let leftovers = #config_name.keys().cloned().collect::<Vec<String>>();
                #output_name.push(format!("the following settings were not recognised: {:#?}", leftovers));
            }
            if !#output_name.is_empty() {
                return Err(std::io::Error::other(format!("{}", #output_name.join("\n"))));
            }
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

pub fn gen_default(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut defaults: TokenStream = TokenStream::new();
    'fields: for field in fields.iter() {
        let attr = &field.attrs;
        let name = match &field.ident {
            Some(value) => value,
            // skip anonymous fields
            None => continue 'fields,
        };
        let ty = &field.ty;
        for attribute in attr {
            match attribute {
                Attribute { meta: Meta::Path(Path{segments, ..}), .. } => {
                    let attr_name = match segments.first() {
                        Some(value) => value,
                        None => panic!("no valid attribute found!"),
                    };
                    if attr_name.ident == "nested_config" {
                        defaults.extend(quote!{
                            #name: #ty::default(),
                        });
                    }
                },
                Attribute { meta: Meta::List(MetaList{ path: Path{ segments, .. }, tokens, .. }), .. } => {
                    let attr_name = match segments.first() {
                        Some(value) => value,
                        None => panic!("no valid attribute found!"),
                    };
                    if attr_name.ident == "default_value" {
                        defaults.extend(quote!{
                            #name: #tokens.into(),
                        });
                    }
                },
                _ => (),
            }
        }
    };
    quote!{
        /// returns an instance with default values
        pub fn default() -> Self {
            Self {
                #defaults
            }
        }
    }
}

pub fn gen_to_string(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut statements: TokenStream = TokenStream::new();
    let mut nested_statements: TokenStream = TokenStream::new();
    let mut comment: TokenStream = TokenStream::new();
    'fields: for field in fields.iter() {
        let attr = &field.attrs;
        let name = match &field.ident {
            Some(value) => value,
            None => continue 'fields,
        };
        let name_string = name.to_string();
        // TODO: continue here
        for attribute in attr {
            if let Attribute{ meta: Meta::Path( Path{segments, ..} ), .. } = attribute {
                // parse nested configs or skip nonconfig elements
                match segments.first() {
                    Some(attr_name) => {
                        if attr_name.ident == "nested_config" {
                            statements.extend(quote! {
                                string.push_str(&format!("\n[{}]\n", #name_string));
                                string.push_str(&self.#name.to_string_nested(&format!("{}", #name_string)));
                            });
                            nested_statements.extend(quote! {
                                string.push_str(&format!("\n[{}.{}]\n", parents, #name_string));
                                string.push_str(&self.#name.to_string_nested(&format!("{}.{}", parents, #name_string)));
                            });
                            continue 'fields;
                        } else if attr_name.ident == "no_config" {
                            continue 'fields;
                        }
                    },
                    None => (),
                }
            } else if let Attribute{ meta: Meta::NameValue( MetaNameValue{path: Path{ segments, .. }, value: Expr::Lit(ExprLit{lit, ..}), ..} ), .. } = attribute {
                // write comments to string
                let attr_type = segments.first().unwrap();
                if attr_type.ident == "doc" {
                    comment = quote!{
                        format!(" #{}", #lit)
                    };
                }
            }
        }
        if comment.is_empty() {
            comment.extend(quote! {""});
        }
        let code = quote!{
            string.push_str(&format!("{} = {}{}\n", #name_string, self.#name.to_string(), #comment));
        };
        statements.extend(code.clone());
        nested_statements.extend(code);
    }
    quote! {
        pub fn to_string(&self) -> String {
            let mut string = String::new();
            #statements
            string
        }

        /// macro function - do not call
        pub fn to_string_nested(&self, parents: &String) -> String {
            let mut string = String::new();
            #nested_statements
            string
        }
    }
}

pub fn gen_config_assignments(fields: &Punctuated<Field, Comma>, config_map_name: &Ident, output_name: &Ident) -> TokenStream {
    let mut assignments: TokenStream = TokenStream::new();
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
            if let Attribute{ meta: Meta::Path( Path{segments, ..} ), .. } = attribute {
                match segments.first() {
                    Some(attr_name) => {
                        if attr_name.ident == "nested_config" {
                            assignments.extend(quote! {
                                match #config_map_name.remove(#name_string) {
                                    Some(ConfigElement::Nested(map)) => {
                                        if let Err(error) = self.#name.parse_from_map(map) {
                                            #output_name.push(error.to_string());
                                        }
                                    },
                                    Some(ConfigElement::Setting(_)) => #output_name.push(format!("`{}` is defined as a nested element, but the configuration file defines it a setting element", #name_string)),
                                    None => #output_name.push(format!("no table `{}` found in config file", #name_string)),
                                }
                            });
                            continue 'fields;
                        } else if attr_name.ident == "no_config" {
                            continue 'fields;
                        }
                    },
                    None => (),
                }
            }
            //} else if let Attribute{ meta: Meta::List()}
        }
        assignments.extend(quote! {
            match #config_map_name.remove(#name_string) {
                Some(ConfigElement::Setting(value)) => {
                    self.#name = match value.parse::<#ty>() {
                        Ok(parsed) => {
                            parsed
                        },
                        Err(_) => {
                            #output_name.push(format!("failed to parse value found for `{}`", #name_string));
                            self.#name.clone()
                        },
                    };
                },
                Some(ConfigElement::Nested(_)) => {
                    #output_name.push(format!("`{}` is a setting element, but the configuration file defines it as a nested element (Table)", #name_string));
                },
                None => {
                    #output_name.push(format!("could not find `{}` in config file", #name_string));
                    self.#name = self.#name.clone();
                },
            };
        });
    }
    assignments
}

