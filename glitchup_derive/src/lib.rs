extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct, FieldsNamed, punctuated::Punctuated, Field};

/// Derives the `Tablefy` trait for any struct.
#[proc_macro_derive(MutConfig)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let sname = &ast.ident;
    let data = &ast.data;

    let fields = extract_fields(&data);
    let types = extract_types(&data);
    let generics = extract_generic_types(&data);

    let insertions = (0..fields.len()).map(|i| {
        let fname = &fields[i].ident;
        let tyident = &types[i].ident;
        let ty_gen = &generics[i];

        let tyname = &tyident.to_string();

        if tyname == "isize" {
            quote! {
                map.insert(String::from(stringify!(#fname)), OInt(self.#fname));
            }
        } else if tyname == "String" {
            quote! {
                map.insert(String::from(stringify!(#fname)), OString(self.#fname));
            }
        } else if tyname == "bool" {
            quote! {
                map.insert(String::from(stringify!(#fname)), OBool(self.#fname));
            }
        } else if tyname == "Vec" {
            let gen_name = &ty_gen[0].ident.to_string();
            let enum_name = match gen_name.as_str() {
                "isize" => quote!{OInt},
                "String" => quote!{OString},
                "bool" => quote!{OBool},
                _ => panic!("'{}' not supported. Please use only supported types.", gen_name.as_str())
            };

            quote! {
                map.insert(String::from(stringify!(#fname)), OArray(self.#fname.iter().map(|x| #enum_name(x)).collect()));
            }
        } else if tyname == "Option" {
            let gen_name = &ty_gen[0].ident.to_string();
            let enum_name = match gen_name.as_str() {
                "isize" => quote!{OInt},
                "String" => quote!{OString},
                "bool" => quote!{OBool},
                _ => panic!("'{}' not supported. Please use only supported types.", gen_name.as_str())
            };

            quote! {
                map.insert(String::from(stringify!(#fname)), self.#fname.map_or(ONone(), |x| #enum_name(x)));
            }
        } else if tyname.find("Config").is_some() {
            quote! {
                map.insert(String::from(stringify!(#fname)), OMap(self.#fname.to_hashmap()));
            }
        } else {
            panic!("Can't use \'{0}\' type - not yet supported by derive(MutConfig).\n\n
                    Hint: If you meant to add a struct implementing MutConfig, please\n
                    name them in the following format: '{0}Config'\n\n
                    Please use one of the supported types as shown below:\n {1:#?}",
                    tyname, ["isize", "String", "bool", "Vec<...>", "Option<...>"]);
        }
    });

    let allinserts = quote! {
        #(#insertions)*
    };

    let expandify = quote! {
        impl MutConfig for #sname {
            fn to_hashmap(&self) -> HashMap<String, MutOptionVal> {
                use MutOptionVal::*;
                let mut map = HashMap::new();

                #allinserts

                map
            }
        }
    };

    // panic!(expandify.to_string());

    expandify.into()
}

/// Extracts the names of the fields of the struct
fn extract_fields(data: &Data) -> &Punctuated<Field, syn::token::Comma>{
    if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed {
            ref named,
            ..
        }), ..
    }) = data {
        named
    } else {
        unimplemented!()
    }
}

/// Extracts the types of the fields of the struct
fn extract_types(data: &Data) -> Vec<&syn::PathSegment>{
    let fields = extract_fields(data);

    // let type_idents: Vec<&syn::Ident> = 
    fields.iter().map(|field| {
        if let syn::Field {
            ty: syn::Type::Path(
                syn::TypePath {
                    path: syn::Path {
                        ref segments,
                        ..
                    },
                    ..
                }
            ),
            ..
        } = field {
            &segments[0]
        } else {
            unimplemented!()
        }
    }).collect::<Vec<&syn::PathSegment>>()
}

/// Extracts the generic arguments of types of fields from the struct.
/// 
/// If a type has no generic arguments, the vector is empty.
/// 
/// The first Vec layer represents the types. The second Vec layer represents the args.
/// 
/// To think about it better, it's like this:
/// 
/// ```
/// output.iter().map(|TYPE| {
///     TYPE.iter().map(|ARG| {
///         ...
///     })
/// })
/// ```
fn extract_generic_types(data: &Data) -> Vec<Vec<&syn::PathSegment>> {
    let types = extract_types(data);

    let something : Vec<Vec<&syn::PathSegment>> = types.iter().map(|ps| {
        let args = 
            if let syn::PathArguments::AngleBracketed(
                syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }
            ) = &ps.arguments {
                Some(args)
            } else {
                None
            };

        let gentype : Vec<&syn::PathSegment> = args.map_or(vec![], |sa| {
            sa.iter().map(|a| {
                if let syn::GenericArgument::Type(syn::Type::Path(
                    syn::TypePath {
                        path : syn::Path {
                            ref segments,
                            ..
                        },
                        ..
                    }
                )) = a {
                    &segments[0]
                } else {
                    unimplemented!()
                }
            }).collect()
        });

        gentype

    }).collect();

    something
}