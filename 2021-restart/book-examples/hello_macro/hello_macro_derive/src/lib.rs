extern crate proc_macro;

// Proc macro, provides a token stream
use proc_macro::TokenStream;
// Quote: turns `syn` data structures back into rust code.
use quote::quote;
// Syn -> parses rust code from a string and creates a data structure.
use syn;

// This will be the macro function
// This function is responsible for parsing the token stream
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Take the input and construct rust code based on the input.
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // Templating from quote!
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // Stringify! is built in rust to convert to literal string
                println!("Hello from macro! I am {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}