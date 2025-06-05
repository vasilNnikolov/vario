use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Result, Token, parse_macro_input};

// Define a struct to hold the parsed macro input
struct ModuleInput {
    file_path: String,
    struct_name: Ident,
    prefix: String,
}

// Implement Parse trait for ModuleInput
impl Parse for ModuleInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse a string literal for the file path
        let file_path_lit: LitStr = input.parse()?;

        // Parse a comma
        input.parse::<Token![,]>()?;

        // Parse an identifier for the struct name
        let struct_name: Ident = input.parse()?;

        // Parse a comma
        input.parse::<Token![,]>()?;

        // Parse a string literal for the prefix
        let prefix_lit: LitStr = input.parse()?;
        let prefix = prefix_lit.value();

        Ok(ModuleInput {
            file_path: file_path_lit.value(),
            struct_name,
            prefix,
        })
    }
}

#[proc_macro]
pub fn generate_module(input: TokenStream) -> TokenStream {
    // Parse the input tokens into our ModuleInput struct
    let ModuleInput {
        file_path,
        struct_name,
        prefix,
    } = parse_macro_input!(input as ModuleInput);

    if !(Path::new(&file_path).exists()) {
        panic!("the path {file_path} does not exist")
    }

    let mod_name: Ident = Ident::new(&prefix, proc_macro2::Span::call_site());

    // For now, we will return an empty module as a placeholder
    let expanded = quote! {
        pub mod #mod_name {
            pub struct #struct_name {
                pub null: ()
            }
        }
    };

    // Convert quoted tokens into a TokenStream
    expanded.into()
}
