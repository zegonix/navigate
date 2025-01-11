#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::quote;

use syn::{Attribute, Error, Field, Meta, Path, punctuated::Punctuated, token::Comma};

pub fn gen_config_load_function(fields: &Punctuated<Field, Comma>, config_map_name: &syn::Ident) -> Result<TokenStream, Error> {
    let mut assignments : TokenStream = TokenStream::new();
    'field_loop: for field in fields.iter() {
        let attr = &field.attrs;
        let name = match &field.ident {
            Some(value) => value,
            // skip anonymous fields
            None => continue 'field_loop,
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
                        continue 'field_loop;
                    } else if value.ident == "no_config" {
                        continue 'field_loop;
                    },
                    None => (),
                }
            }
        }
        assignments.extend(quote! {
            self.#name = match #config_map_name.get(#name_string) {
                Some(value) => match value.parse::<#ty>() {
                    Ok(parsed) => {
                        _ = #config_map_name.remove(#name_string);
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
    Ok(assignments.into())
}

