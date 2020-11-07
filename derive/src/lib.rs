extern crate proc_macro;

use syn;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::*;


#[proc_macro_derive(Arbitrary)]
pub fn hello_macro_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data{
        Data::Struct(data) => {
            Ok(
                data.fields.iter()
                .map(|i| {
                    let name = i.ident.clone().unwrap();
                    quote! {#name }
                })
                .collect::<Vec<_>>()
            )
        },
        _ => Err(())
    }.expect("WE ONLY SUPPORT STRUCTS");

    let to_bytes = match &ast.data{
        Data::Struct(data) => {
            Ok(
                data.fields.iter()
                .map(|i| {
                    let field = i.ident.clone().unwrap();
                    quote! {
                        result.append(
                            &mut self.#field.to_bytes()
                        );
                    }
                })
                .collect::<Vec<_>>()
            )
        },
        _ => Err(())
    }.expect("WE ONLY SUPPORT STRUCTS");

    let from_bytes = match &ast.data{
        Data::Struct(data) => {
            Ok(
                data.fields.iter()
                .map(|i| {
                    let field = i.ident.clone().unwrap();
                    let ty = i.ty.clone();
                    quote! {
                        let (#field, data) = #ty::build_from_bytes(data); 
                    }
                })
                .collect::<Vec<_>>()
            )
        },
        _ => Err(())
    }.expect("WE ONLY SUPPORT STRUCTS");

    let gen = quote! {
        impl Arbitrary for #name {
            fn to_bytes(&self) -> Vec<u8>{
                let mut result = Vec::new();
                #(#to_bytes)*

                result
            }
            fn build_from_bytes(data: &[u8]) -> (Self, &[u8]){
                #(#from_bytes)*
                (
                    #name{
                        #(#fields),*
                    },
                    data
                )
            }
        }
    
    };
    gen.into()
}
