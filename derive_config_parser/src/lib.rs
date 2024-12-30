
use core::panic;
use proc_macro::TokenStream;

/// **for structs only**
/// - implements `parse_config(&mut self, input: &String) -> Result<()>`
/// which parses a string and fills the fills recognised values into the struct
/// - implements `write_default_config() -> Result<String>`
/// which write a default configuration, in case the documentation is lacking
#[proc_macro_derive(ConfigParser)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct{ fields: syn::Fields::Named(syn::FieldsNamed{ ref named, .. }), .. }) = ast.data {
        named
    } else {
        panic!("failed to parse struct fields");
    };

    println!("{:#?}", ast);
//    let string = format!("{:#?}", ast);
//    _ = std::fs::write("test.txt", string);

    let expanded_stream: TokenStream = quote::quote! {
        impl #name {
            pub fn parse_config(&mut self, input: &String) -> std::io::Result<()> {
                let mut setting_names = std::collections::HashMap::new();
                let lines = input.lines();
                for line in lines {
                    let mut tokens: Vec<&str> = line.split('=').map(|entry| entry.trim()).collect();
                    tokens.retain(|entry| !entry.is_empty());
                    if tokens.len() != 2 {
                        // println!("error in line'", line);
                        continue;
                    }
                    setting_names.insert(tokens[0], tokens[1]);
                }
                //println!("{:#?}", setting_names);
                Ok(())
            }
        }
    }.into();
    expanded_stream
//    TokenStream::new()
}
