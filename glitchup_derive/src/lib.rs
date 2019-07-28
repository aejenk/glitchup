extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct, FieldsNamed, punctuated::Punctuated, Field};

/// Derives the `MutConfig` trait for any struct.
#[proc_macro_derive(MutConfig)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let sname = &ast.ident;
    let data = &ast.data;

    if sname.to_string().find("Config").is_none() {
        panic!("Please name the struct in a format like <name>Config (ex. MainConfig)");
    };

    let fields = filter_ignore_out(extract_fields(&data));
    let types = extract_types(&data);
    let generics = extract_generic_types(&data);


    // generates an appropriate map insertion according to the field
    let insertions = (0..fields.len()).map(|i| {
        let fname = &fields[i].ident;
        let tyident = &types[i].ident;
        let ty_gen = &generics[i];

        let tyname = &tyident.to_string();

        // isize, String, and bool are all simple cases
        if tyname == "isize" {
            quote! {
                map.insert(String::from(stringify!(#fname)), OInt(self.#fname.clone()));
            }
        } else if tyname == "String" {
            quote! {
                map.insert(String::from(stringify!(#fname)), OString(self.#fname.clone()));
            }
        } else if tyname == "bool" {
            quote! {
                map.insert(String::from(stringify!(#fname)), OBool(self.#fname.clone()));
            }
        // if the field type is a vector, it checks the type of the generic
        // if it's supported, it then converts the Vector into OArray(Vec<MutOptionVal>)
        } else if tyname == "Vec" {
            let gen_name = &ty_gen[0].ident.to_string();
            let enum_name = match gen_name.as_str() {
                "isize" => quote!{OInt},
                "String" => quote!{OString},
                "bool" => quote!{OBool},
                _ => {
                    incompatible_type_panic(&tyname);
                    quote!{}
                }
            };

            quote! {
                map.insert(String::from(stringify!(#fname)), OArray(self.#fname.iter().map(|x| #enum_name(x.clone())).collect()));
            }
        // If the field type is an Option, it checks the type of the generic
        // if it's supported, it then either converts the value into an ONone or its appropriate MutOptionVal
        } else if tyname == "Option" {
            let gen_name = &ty_gen[0].ident.to_string();
            let enum_name = match gen_name.as_str() {
                "isize" => quote!{OInt},
                "String" => quote!{OString},
                "bool" => quote!{OBool},
                _ => {
                    incompatible_type_panic(&tyname);
                    quote!{}
                }
            };

            quote! {
                map.insert(String::from(stringify!(#fname)), self.#fname.clone().map_or(ONone(), |x| #enum_name(x.clone())));
            }
        } else if tyname.find("Config").is_some() {
            quote! {
                map.insert(String::from(stringify!(#fname)), OMap(self.#fname.to_hashmap()));
            }
        } else {
            incompatible_type_panic(&tyname);
            quote!{}
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

fn not_ignored(field: &Field) -> bool {
    for attr in &field.attrs {
        let segs = &attr.path.segments;

        for seg in segs {
            let attrname = seg.ident.to_string();

            if attrname == "ignore" {return false;};
        }
    };

    true
}

fn filter_ignore_out(fields: &Punctuated<Field, syn::token::Comma>) -> Vec<&Field> {
    fields.iter().filter(|f| not_ignored(*f)).collect()
}

/// Extracts the types of the fields of the struct
fn extract_types(data: &Data) -> Vec<&syn::PathSegment>{
    let fields = filter_ignore_out(extract_fields(data));

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
            eprintln!("References are not supported with #[derive(MutConfig)]. Reference found within type.");
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
                    eprintln!("References are not supported with #[derive(MutConfig)]. Reference found within generic.");
                    unimplemented!()
                }
            }).collect()
        });

        gentype

    }).collect();

    something
}

/// Panics if the type name isn't compatible with the macro.
/// 
/// To be used by `derive` to avoid repetition.
fn incompatible_type_panic(tyname: &String) {
    panic!("Can't use \'{0}\' type - not yet supported by derive(MutConfig).\nHint: If you meant to add a struct implementing MutConfig, please name them in the following format: '{0}Config'\nPlease use one of the supported types as shown below:\n {1:#?}",tyname, ["isize", "String", "bool", "Vec<...>", "Option<...>"]);
}