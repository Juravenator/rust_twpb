// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

extern crate proc_macro;
extern crate proc_macro2;
use paste::paste;
use proc_macro::{TokenStream};
use proc_macro2::{Span};
use quote::{quote, format_ident, ToTokens};
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DataEnum, DeriveInput, Lit,
    Field, Fields, Ident, Path, Type, TypePath, FieldsUnnamed, FieldsNamed,MetaList, Meta, NestedMeta,
};

#[derive(Debug)]
struct ParsedField {
    pub field_name: syn::Ident,
    pub field_numbers: Vec<u32>,
    pub field_type: syn::Type,
    pub proto_type: String,
}

#[derive(Debug)]
struct ParsedVariant {
    pub field_name: syn::Ident,
    pub field_numbers: Vec<u32>,
    pub field_type: syn::Type,
    pub proto_type: String,
}

impl ParsedVariant {
    fn parse(field: syn::Variant) -> syn::parse::Result<Self> {
        // println!("parse enum variant {:#?}", field.fields);
        let field_type = match field.fields {
            syn::Fields::Unnamed(syn::FieldsUnnamed{unnamed: fields, ..}) => fields.into_iter().nth(0),
            _ => panic!("no enum variant type {}", field.ident),
        };
        let field_type = match field_type {
            Some(field_type) => field_type.ty,
            _ => panic!("no enum variant type"),
        };
        let mut result = ParsedVariant{
            field_name: field.ident,
            field_numbers: vec![0],
            field_type: field_type,
            proto_type: "".to_owned(),
        };
        // if let syn::Fields::Unnamed(syn::FieldsUnnamed{unnamed: fields, ..}) = field.fields {
        //     // result.field_type = Some(fields[0].ty);
        //     if let syn::Type::Path(syn::TypePath{path: p, ..}) = fields[0].ty {
        //         result.field_type = Some(p);
        //     }
        // }
        // result.field_name = field.ident;

        let mut twpb_attr: Vec<_> = field.attrs
            .into_iter()
            .filter(|a| {println!("path {:?}", a.path);a.path.is_ident("twpb")})
            .collect();

        let twpb_attr = match twpb_attr.len() {
            1 => &twpb_attr[0],
            0 => panic!("All fields of a message must specify a #[twpb] attribute, missing for field '{:?}'", result.field_name),
            n => panic!("A field can specify a #[twpb] attribute only once. Field '{:?}' specified it {} times.", result.field_name, n),
        };

        // start parsing the ?? part of #[twpb(??)]
        let metas = match twpb_attr.parse_meta()? {
            Meta::List(l) => l.nested,
            // One can also write other attributes, like '#[twpb = value]'.
            // We don't do that here.
            _ => panic!("twpb attribute can only be of the form '#[twpb(..)]': {:?}", twpb_attr),
        };
        
        for meta in metas {
            match meta {
                // parse the field number
                NestedMeta::Meta(Meta::NameValue(ref nv)) if nv.path.is_ident("nr") => {
                    if let Lit::Int(li) = &nv.lit {
                        result.field_numbers = vec![li.base10_parse::<u32>()?];
                    } else if let Lit::Str(ls) = &nv.lit {
                        let numbers: Vec<u32> = ls.value().split(";")
                        .map(|s| {
                            if let Some(div) = s.find('-') {
                                let left = &s[..div];
                                let right = &s[div+1..];
                                let left = left.parse::<u32>().unwrap_or_else(|_| panic!("nr range must be formatted as `^[0-9]+-[0-9]+$`, got '{}'", s));
                                let right = right.parse::<u32>().unwrap_or_else(|_| panic!("nr range must be formatted as `^[0-9]+-[0-9]+$`, got '{}'", s));
                                return (left..=right).collect::<Vec<_>>();
                            } else {
                                let s = s.parse::<u32>().unwrap_or_else(|_| panic!("invalid number, got '{}'", s));
                                return vec![s];
                            }
                        })
                        .flatten().collect();
                        result.field_numbers = numbers;
                    } else {
                        panic!("nr must specify a number");
                    }
                }

                // parse the field type
                NestedMeta::Meta(Meta::Path(ref p)) => {
                    if let Some(ident) = p.get_ident() {
                        let s = ident.to_string();
                        match s.as_ref() {
                            // someone likes numbers...
                            "int32" | "int64" |
                            "uint32" | "uint64" |
                            "sint32" | "sint64" |
                            "fixed32" | "fixed64" |
                            "sfixed32" | "sfixed64" |
                            "double" | "float" |
                            // non-numbers whatever
                            "bool" | "string" | "bytes" | "oneof" |
                            // special case, embedded messages
                            "message" => result.proto_type = s.to_owned(),
                            _ => panic!("unknown field type '{}'", s),
                        }
                    }
                    // result.proto_type = ParsedFieldProtoType::parse(p.get_ident()
                    //     .expect("Unexpected absence of name for attribute"))
                }

                // NestedMeta::Meta(Meta::Path(ref p)) if p.is_ident("string") => {
                //     println!("it's a string");
                // }
                _ => panic!("invalid attribute: {:?}", twpb_attr),
            }
        }

        Ok(result)
    }
}

impl ParsedField {
    fn parse(field: syn::Field) -> syn::parse::Result<Self> {

        // let mut result: ParsedField = Default::default();
        // result.field_name = field.ident;
        let mut result = ParsedField{
            field_name: field.ident.expect("Field has no name"),
            field_numbers: vec![0],
            proto_type: "".to_owned(),
            field_type: field.ty,
        };


        let mut twpb_attr: Vec<_> = field.attrs
            .into_iter()
            // .filter(|a| {println!("path {:?}", a.path);a.path.is_ident("twpb")})
            .filter(|a| a.path.is_ident("twpb"))
            .collect();

        let twpb_attr = match twpb_attr.len() {
            1 => &twpb_attr[0],
            0 => panic!("All fields of a message must specify a #[twpb] attribute, missing for field '{:?}' of type '{:}'", result.field_name, result.field_type.to_token_stream()),
            n => panic!("A field can specify a #[twpb] attribute only once. Field '{:?}' specified it {} times.", result.field_name, n),
        };

        // start parsing the ?? part of #[twpb(??)]
        let metas = match twpb_attr.parse_meta()? {
            Meta::List(l) => l.nested,
            // One can also write other attributes, like '#[twpb = value]'.
            // We don't do that here.
            _ => panic!("twpb attribute can only be of the form '#[twpb(..)]': {:?}", twpb_attr),
        };
        
        for meta in metas {
            match meta {
                // parse the field number
                NestedMeta::Meta(Meta::NameValue(ref nv)) if nv.path.is_ident("nr") => {
                    if let Lit::Int(li) = &nv.lit {
                        result.field_numbers = vec![li.base10_parse::<u32>()?];
                    } else if let Lit::Str(ls) = &nv.lit {
                        let numbers: Vec<u32> = ls.value().split(";")
                        .map(|s| {
                            if let Some(div) = s.find('-') {
                                let left = &s[..div];
                                let right = &s[div+1..];
                                let left = left.parse::<u32>().unwrap_or_else(|_| panic!("nr range must be formatted as `^[0-9]+-[0-9]+$`, got '{}'", s));
                                let right = right.parse::<u32>().unwrap_or_else(|_| panic!("nr range must be formatted as `^[0-9]+-[0-9]+$`, got '{}'", s));
                                return (left..=right).collect::<Vec<_>>();
                            } else {
                                let s = s.parse::<u32>().unwrap_or_else(|_| panic!("invalid number, got '{}'", s));
                                return vec![s];
                            }
                        })
                        .flatten().collect();
                        result.field_numbers = numbers;
                    } else {
                        panic!("nr must specify a number");
                    }
                }

                // parse the field type
                NestedMeta::Meta(Meta::Path(ref p)) => {
                    if let Some(ident) = p.get_ident() {
                        let s = ident.to_string();
                        match s.as_ref() {
                            // someone likes numbers...
                            "int32" | "int64" |
                            "uint32" | "uint64" |
                            "sint32" | "sint64" |
                            "fixed32" | "fixed64" |
                            "sfixed32" | "sfixed64" |
                            "double" | "float" |
                            // non-numbers whatever
                            "bool" | "string" | "bytes" | "oneof" => result.proto_type = s.to_owned(),
                            _ => (),
                        }
                    }
                    // result.proto_type = ParsedFieldProtoType::parse(p.get_ident()
                    //     .expect("Unexpected absence of name for attribute"))
                }

                // NestedMeta::Meta(Meta::Path(ref p)) if p.is_ident("string") => {
                //     println!("it's a string");
                // }
                _ => panic!("invalid attribute: {:?}", twpb_attr),
            }
        }

        Ok(result)
    }
}

#[proc_macro_derive(Enum, attributes(twpb))]
pub fn derive_enum(tokens: TokenStream) -> TokenStream {
    try_derive_enum(tokens).unwrap()
}


fn try_derive_enum(tokens: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse(tokens)?;

    // let name = input.ident;
    let struct_name = input.ident;
    println!("derive enum {}", struct_name);

    let variants = match input.data {
        Data::Enum(DataEnum{variants, ..}) => variants,
        _ => panic!("Derive enum called on non-enum type"),
    };

    let mut q = quote!();
    let mut a = quote!();
    for variant in variants {
        println!("variant {}", variant.ident);

        // let fields: Result<Vec<_>, _> = variant.fields.into_iter()
        //     .map(|field| ParsedField::parse_enum_variant(variant))
        //     .collect();
        let field = ParsedVariant::parse(variant)?;
        let field_name = field.field_name;
        let proto_type = field.proto_type;
        let field_type = field.field_type;
        let field_numbers = field.field_numbers.iter().map(|n| quote!(#n)).reduce(|acc, new| quote! {#acc , #new});
        println!("'{}' of type {:?} has field numbers {:?}", 
            field_name, proto_type, field.field_numbers);
        q.extend(quote!{
            println!("Dealing with variant '{}::{}' ({}) at field numbers [{}]",
                stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type), stringify!(#field_numbers));
        });

        if proto_type == "oneof" {
            panic!("nested oneof unimplemented")
        } else if proto_type == "message" {
            // panic!("encountered embedded message for {}, {:?}", field_name, field_type);
            a.extend(quote!{
                println!("testing for embedded message match '{}::{}' [{}] = '{}'", stringify!(#struct_name), stringify!(#field_name), stringify!(#field_numbers), stringify!(#field_type));
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    let bufsize = ::twpb::decode_leb128_u32(&mut bytes)?;
                    println!("embedded message match with size {}", bufsize);
                    let value = #struct_name::#field_name(#field_type::twpb_decode_iter(&mut bytes)?);
                    return Ok(value);
                }
            });
        } else {
            let parse_fn = Ident::new(&format!("decode_{}", &proto_type), Span::call_site());
            a.extend(quote!{
                println!("testing for match '{}::{}' [{}]", stringify!(#struct_name), stringify!(#field_name), stringify!(#field_numbers));
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    println!("enum variant match for '{}::{}' ({})", stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type));
                    // let value = ::twpb::#parse_fn(&mut bytes, stringify!(#struct_name::#field_name))?;
                    // result.#field_name = #struct_name(value);
                    // return Ok(#struct_name::test(::heapless::String::from("ha")));
                    let value = #struct_name::#field_name(::twpb::#parse_fn(&mut bytes, stringify!(#struct_name::#field_name))?);
                    return Ok(value);
                }
            });
        }
    }

    // println!("enum fields {:?}", fields);
    Ok(TokenStream::from(quote!{
        impl #struct_name {
            pub fn twpb_decode<'a, I>(field_number: u32, wire_type: u8, mut bytes: I, field_name: &str) -> Result<#struct_name, ::twpb::DecodeError>
            where I: Iterator<Item = &'a u8> {
                println!("decoding proto {}", stringify!(#struct_name));

                #q

                // let result = match ::twpb::decode_tag(&mut bytes) {
                //     Ok((field_number, wire_type)) => {
                        println!("got field nr {}", field_number);
                        println!("got wire type {}", wire_type);
                        #a
                        return Err(::twpb::DecodeError::UnexpectedEndOfBuffer);
                        // return Ok(result);
                //     },
                //     // Err(::twpb::DecodeError::EmptyBuffer) => break, // this only applies to message, not oneof
                //     Err(e) => return Err(e),
                // };
                // result
                // Ok(#struct_name::test(::heapless::String::from("ha")))
            }
        }
    }))
}

#[proc_macro_derive(Message, attributes(twpb))]
pub fn derive_message(tokens: TokenStream) -> TokenStream {
    try_derive_message(tokens).unwrap()
}

fn try_derive_message(tokens: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse(tokens)?;

    // let name = input.ident;
    let struct_name = input.ident;

    // Get all struct fields
    let fields = match input.data {
        Data::Struct(DataStruct{fields: Fields::Named(FieldsNamed{named: fields, ..}), ..})
            => fields.into_iter().collect(),
        // There can also be no fields at all
        Data::Struct(DataStruct{fields: Fields::Unit, ..}) => vec![],

        Data::Struct(DataStruct{fields: Fields::Unnamed(..), ..})
            => panic!("Unnamed fields are not supported"),
        Data::Enum(..) => panic!("Message can not be derived for an enum"),
        Data::Union(..) => panic!("Message can not be derived for a union"),
    };

    // println!("derive message fields {:?}", fields);

    // Parse each field and extract protobuf info
    let fields: Result<Vec<_>, _> = fields.into_iter()
        // .map(|field| {println!("a message field {:?}", field.ty);ParsedField::parse(field)})
        .map(|field| ParsedField::parse(field))
        .collect();
    let fields = fields?;

    let mut q = quote!();
    let mut a = quote!();
    for field in fields {
        println!("'{}::{:?}' of type {:?} has field numbers {:?}",
            struct_name, field.field_name, field.proto_type, field.field_numbers);

        let field_name = field.field_name;
        let proto_type = field.proto_type;
        let field_numbers = field.field_numbers.iter().map(|n| quote!(#n)).reduce(|acc, new| quote! {#acc , #new});
        q.extend(quote!{
            println!("Dealing with '{}::{}' ({}) at field numbers [{}]",
                stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type), stringify!(#field_numbers));
        });

        if proto_type == "oneof" {
            // println!("message encountered enum, skipping. '{:#?}'", field.field_type.unwrap());
            let b = match field.field_type {
                syn::Type::Path(syn::TypePath{path: syn::Path{segments: b, ..}, ..}) => 
                b.into_iter().find(|b| b.ident == "Option" && ! b.arguments.is_empty()).unwrap().arguments,
                _ => panic!("b"),
            };

            let b = match b {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args: b, ..}) => b,
                _ => panic!("b"),
            };

            let b = &b[0];
            println!("message encountered enum for {:?}", b);
            // a.extend(quote!{println!("I AM {}", stringify!(#b))});
            a.extend(quote!{
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    println!("parsing enum field of type '{}'", stringify!(#b));
                    println!("match for '{}::{}' ({})", stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type));
                    result.#field_name = #b::twpb_decode(field_number, wire_type, &mut bytes, stringify!(#field_name)).ok();
                }
            });

        } else if proto_type == "message" {
            panic!("encountered embedded message for {}, {}", field_name, proto_type);

        } else {
            let parse_fn = Ident::new(&format!("decode_{}", &proto_type), Span::call_site());
            a.extend(quote!{
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    println!("match for '{}::{}' ({})", stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type));
                    result.#field_name = ::twpb::#parse_fn(&mut bytes, stringify!(#field_name))?;
                }
            });
        }
    }

    // // For each field in the struct
    // for field in fields {
    //     // println!("field {:?}", field.ident);
    //     // let mut field_number: u32 = 0;

    //     let parsed = ParsedField::parse(field)?;
    //     println!("'{}::{}' of type {:?} has field number {:?}", 
    //         struct_name, parsed.field_name, parsed.proto_type, parsed.field_number);



        
    // }

    Ok(TokenStream::from(quote!{
        impl #struct_name {
            pub fn twpb_decode(buf: &[u8]) -> Result<#struct_name, ::twpb::DecodeError> {
                #struct_name::twpb_decode_iter(buf.iter())
            }

            pub fn twpb_decode_iter<'a, I>(mut bytes: I) -> Result<#struct_name, ::twpb::DecodeError>
            where I: Iterator<Item = &'a u8> {
                println!("decoding proto {}", stringify!(#struct_name));
                let mut result = #struct_name::default();

                #q

                // Protobuf messages are a list of key->value pairs, the key being a tag
                // which consists of the field type and the wire type.
                // As long as keys keep being encountered in the buffer, read said keys and values.
                loop {
                    match ::twpb::decode_tag(&mut bytes) {
                        Ok((field_number, wire_type)) => {
                            println!("got field nr {}", field_number);
                            println!("got wire type {}", wire_type);
                            #a
                            // return Ok(result);
                        },
                        Err(::twpb::DecodeError::EmptyBuffer) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
        }
    }))
}
