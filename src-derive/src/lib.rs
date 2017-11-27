#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, VariantData};

#[proc_macro_derive(FromRouteParams)]
pub fn from_route_params(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    // Parse the string representation into a syntax tree
    let ast = syn::parse_macro_input(&source).unwrap();

    // create a vector containing the names of all fields on the struct
    let mut tys : Vec<syn::Ty> = Vec::new();
    let idents: Vec<Ident> = match ast.body {
        syn::Body::Struct(vdata) => {
            match vdata {
                VariantData::Struct(fields) => {
                    let mut idents = Vec::new();
                    for ref field in fields.iter() {
                        match &field.ident {
                            &Some(ref ident) => {
                                idents.push(ident.clone());
                                tys.push(field.ty.clone())
                            },
                            &None => panic!("Your struct is missing a field identity!"),
                        }
                    }
                    idents
                },
                VariantData::Tuple(_) | VariantData::Unit => {
                    panic!("You can only derive this for normal structs!");
                },
            }
        },
        syn::Body::Enum(_) => panic!("You can only derive this on structs!"),
    };

    // contains quoted strings containing the struct fields in the same order as
    // the vector of idents.
    let mut keys = Vec::new();
    for ident in idents.iter() {
        keys.push(String::from(ident.as_ref()));
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let idents_1 = idents.clone();

    let tokens = quote! {
        impl #impl_generics FromRouteParams<#name> for #name #ty_generics #where_clause {
            fn from_params<'a>(params: &::router::Params) -> ::iron::IronResult<#name> {
                // start with the default implementation
                #(
                    let key = #keys;
                    let #idents = match params.find(key) {
                        None => return Err(::iron::error::IronError::new(
                            ::util::ClientError::MissingRouteParam(key.to_owned()),
                            ::iron::status::BadRequest)
                        ),
                        Some(val) => match #tys::from_str(val) {
                            Err(err) => return Err(::iron::error::IronError::new(err, err.description())),
                            Ok(val) => val,
                        },
                    };
                )*

                ::std::result::Result::Ok(#name {
                    #(
                        #idents_1,
                    )*
                })
            }
        }
    };

    tokens.parse().unwrap()
}

#[proc_macro_derive(FromBodyParser)]
pub fn from_bodyparser(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    // Parse the string representation into a syntax tree
    let ast = syn::parse_macro_input(&source).unwrap();

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics FromBodyParser<#name> for #name #ty_generics #where_clause {
            fn from_request<'a>(req: &::iron::Request) -> ::std::result::Result<#name, ::bodyparser::BodyError> {
                // start with the default implementation
                 req.get::<bodyparser::Struct<#name>>()
            }
        }
    };

    tokens.parse().unwrap()
}