extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct MacroInput {
    file_name: LitStr,
    struct_name: Ident,
    module_name: Ident,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file_name: LitStr = input.parse().inspect_err(|_| {
            eprintln!("Could not parse the file name out of the proc macro input")
        })?;
        input.parse::<Token![,]>()?;
        let struct_name: Ident = input
            .parse()
            .inspect_err(|_| eprintln!("Could not parse struct_name out of macro input"))?;
        input.parse::<Token![,]>()?;
        let module_name: Ident = input.parse()?;
        Ok(MacroInput {
            file_name,
            struct_name,
            module_name,
        })
    }
}

#[proc_macro]
pub fn find_struct(input: TokenStream) -> TokenStream {
    let MacroInput {
        file_name,
        struct_name,
        module_name,
    } = parse_macro_input!(input as MacroInput);

    // Read the file contents
    let file_path = std::path::Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR not set"))
        .join(file_name.value());
    let file_content = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {}", file_path.display()));

    // Parse file content as a Rust AST
    let syntax_tree = syn::parse_file(&file_content).expect("Unable to parse file content");

    // Find the struct with the specified name
    for item in syntax_tree.items {
        if let syn::Item::Struct(ref struct_item) = item {
            if struct_item.ident == struct_name {
                // Return the struct definition as a TokenStream
                return TokenStream::from(quote! {
                    pub mod #module_name {
                        #struct_item
                        pub const MODCONST: u8=1;
                    }
                });
            }
        }
    }

    panic!("Struct {} not found in {}", struct_name, file_name.value());
}
