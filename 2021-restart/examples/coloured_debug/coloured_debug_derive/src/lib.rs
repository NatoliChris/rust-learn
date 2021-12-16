extern crate proc_macro;
extern crate colored;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ColouredDebug)]
pub fn coloured_debug_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_coloured_debug(&ast)
}

fn impl_coloured_debug(ast: &syn::DeriveInput) -> TokenStream {
    let name: &syn::Ident = &ast.ident;
    // Attributes?
    let attrs: &Vec<syn::Attribute> = &ast.attrs;

    let gen = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{} {}",
                    "DEBUG".red().bold(),
                    format!(
                        "{} {{ {} }}",
                        stringify!(#name),
                        stringify!(#(#attrs),*)
                    ).blue()
                )
            }
        }
    };

    gen.into()
}
