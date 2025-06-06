use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct MacroInput {
    /// path to the generated bindings file, inside "OUT_DIR". If the file is generated in env!("OUT_DIR")/a.rs, this should be "a.rs"
    file_name: LitStr,
    /// name of the struct you want to find
    struct_name: Ident,

    /// a string that is at the beginning of all constants. Ex if `constant_start: "RCC_"`, then all constants staring with `RCC_` will be added into `module_name`
    constant_start: LitStr,
    // /// name of the <STRUCT>_BASE u32 constant that shows the location of the peripheral
    // base_name: Ident,
    /// name of the generated module
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
        let constant_start: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        // let base_name: Ident = input.parse()?;
        // input.parse::<Token![,]>()?;
        let module_name: Ident = input.parse()?;
        Ok(MacroInput {
            file_name,
            struct_name,
            // base_name,
            constant_start,
            module_name,
        })
    }
}

#[proc_macro]
pub fn find_struct(input: TokenStream) -> TokenStream {
    let MacroInput {
        file_name,
        struct_name,
        // base_name,
        constant_start,
        module_name,
    } = parse_macro_input!(input as MacroInput);

    let file_path = std::path::Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR not set"))
        .join(file_name.value());
    let file_content = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {}", file_path.display()));

    let syntax_tree = syn::parse_file(&file_content).expect("Unable to parse file content");

    // get structs whose name matches struct_name
    let structs_with_matching_name: Vec<_> = syntax_tree
        .items
        .iter()
        .filter(|&item| match item {
            syn::Item::Struct(s) => s.ident == struct_name,
            _ => false,
        })
        .collect();

    assert!(
        structs_with_matching_name.len() == 1,
        "There are multiple structs whose ident is {struct_name}"
    );
    let struct_item = structs_with_matching_name[0];

    let constants_starting_with_str: Vec<_> = syntax_tree
        .items
        .iter()
        .filter(|&item| match item {
            syn::Item::Const(c) => c.ident.to_string().starts_with(&constant_start.value()),
            _ => false,
        })
        .collect();

    TokenStream::from(quote! {
        pub mod #module_name {
            #struct_item

            impl #struct_name {
                pub fn new_static_ref() -> &'static mut Self {
                    unsafe {
                        let ptr = 0x1234 as *mut Self;
                        &mut *ptr
                    }
                }
            }

            #(#constants_starting_with_str)*
        }
    })
}
