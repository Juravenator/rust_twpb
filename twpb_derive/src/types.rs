use quote::ToTokens;
use syn::{self, Lit, Meta, NestedMeta};

#[derive(Debug)]
pub struct ParsedField {
    pub field_name: syn::Ident,
    pub field_numbers: Vec<u32>,
    pub field_type: syn::Type,
    pub proto_type: String,
    pub repeated: bool,
}

#[derive(Debug)]
pub struct ParsedVariant {
    pub field_name: syn::Ident,
    pub field_numbers: Vec<u32>,
    pub field_type: syn::Type,
    pub proto_type: String,
}


impl ParsedVariant {
    pub fn parse(field: syn::Variant) -> syn::parse::Result<Self> {
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

        let twpb_attr: Vec<_> = field.attrs
            .into_iter()
            .filter(|a| a.path.is_ident("twpb"))
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
                }

                _ => panic!("invalid attribute: {:?}", twpb_attr),
            }
        }

        Ok(result)
    }
}

impl ParsedField {
    pub fn parse(field: syn::Field) -> syn::parse::Result<Self> {

        let mut result = ParsedField{
            field_name: field.ident.expect("Field has no name"),
            field_numbers: vec![0],
            proto_type: "".to_owned(),
            field_type: field.ty,
            repeated: false,
        };


        let twpb_attr: Vec<_> = field.attrs
            .into_iter()
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
                            "repeated" => result.repeated = true,
                            _ => panic!("unknown field type '{}'", s),
                        }
                    }
                }

                _ => panic!("invalid attribute: {:?}", twpb_attr),
            }
        }

        Ok(result)
    }
}