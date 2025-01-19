mod generator_functions;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

use generator_functions::*;

/// **for structs only**
/// - implements `parse_config(&mut self, input: &String) -> Result<()>`
/// which parses a string and fills the fills recognised values into the struct
/// - implements `write_default_config() -> Result<String>`
/// which write a default configuration, in case the documentation is lacking
#[proc_macro_derive(
    ConfigParser,
    attributes(
        default_value,
        nested_config,
        no_config,
        style_config
    )
)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let config_name = syn::Ident::new("config", name.span());
    let output_name = syn::Ident::new("output", name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct{ fields: syn::Fields::Named(syn::FieldsNamed{ ref named, .. }), .. }) = ast.data {
        named
    } else {
        panic!("the macro `ConfigParser` applies only to structs!");
    };
    let assignments: TokenStream = gen_config_assignments(fields, &config_name, &output_name);
    let func_parse_string: TokenStream = gen_parse_from_string(&config_name, &output_name, &assignments);
    let func_parse_map: TokenStream = gen_parse_from_map(&config_name, &output_name, &assignments);
    let func_to_ansi_sequences: TokenStream = gen_to_ansi_sequences(fields);
    let func_default: TokenStream = gen_default(fields);

    quote! {
        impl #name {
            #func_parse_string
            #func_parse_map
            #func_to_ansi_sequences
            #func_default

            // TODO: implement function to parse style settings
        }
    }.into()
}

