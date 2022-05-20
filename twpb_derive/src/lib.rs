mod types;
mod wiretypes;

use types::*;
use wiretypes::wire_types;

extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::{TokenStream};
use proc_macro2::{Span};
use quote::quote;
use syn::{self, Data, DataStruct, DataEnum, DeriveInput, Fields, Ident};


#[proc_macro_derive(Enum, attributes(twpb))]
pub fn derive_enum(tokens: TokenStream) -> TokenStream {
    try_derive_enum(tokens).unwrap()
}


fn try_derive_enum(tokens: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse(tokens)?;

    let struct_name = input.ident;
    // println!("derive enum {}", struct_name);

    let variants = match input.data {
        Data::Enum(DataEnum{variants, ..}) => variants,
        _ => panic!("Derive enum called on non-enum type"),
    };

    let mut debugmsg = quote!();
    let mut decodecode = quote!();
    let mut encodecode = quote!();
    for variant in variants {
        // println!("variant {}", variant.ident);

        let field = ParsedVariant::parse(variant)?;
        let field_name = field.field_name;
        let proto_type = field.proto_type;
        let field_type = field.field_type;
        let field_numbers = field.field_numbers.iter().map(|n| quote!(#n)).reduce(|acc, new| quote! {#acc , #new});
        let first_field_number = field.field_numbers[0];
        // println!("'{}' of type {:?} has field numbers {:?}", 
        //     field_name, proto_type, field.field_numbers);
        debugmsg.extend(quote!{
            // println!("Dealing with variant '{}::{}' ({}) at field numbers [{}]",
            //     stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type), stringify!(#field_numbers));
        });

        if proto_type == "oneof" {
            panic!("nested oneof unimplemented")
        } else if proto_type == "message" {
            decodecode.extend(quote!{
                // println!("testing for embedded message match '{}::{}' [{}] = '{}'", stringify!(#struct_name), stringify!(#field_name), stringify!(#field_numbers), stringify!(#field_type));
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    let bufsize = ::twpb::decoder::leb128_u32(&mut bytes)?;
                    // println!("embedded message match with size {}", bufsize);
                    let iterator = ::twpb::LimitedIterator::new(&mut bytes, bufsize);
                    let value = #struct_name::#field_name(#field_type::twpb_decode_iter(iterator)?);
                    return Ok(value);
                }
            });
            encodecode.extend(quote!{
                #struct_name::#field_name(c) => {
                    bytes_written += ::twpb::encoder::tag(&mut buffer, &#first_field_number, &::twpb::wire_types::LENGTHDELIMITED)?;
                    // We need to send the payload size first.
                    // So serialize twice, once to a nil buffer just to count bytes
                    let mut nullbuffer = ::twpb::iterators::NullCounterBuffer::new();
                    let len = c.twpb_encode(&mut nullbuffer)?;
                    bytes_written += ::twpb::encoder::leb128_u32(&mut buffer, &(len as u32))?;
                    // second time to actually send data
                    bytes_written += c.twpb_encode(&mut buffer)?;
                },
            });
        } else {
            let parse_fn = Ident::new(&format!("{}", &proto_type), Span::call_site());
            decodecode.extend(quote!{
                // println!("testing for match '{}::{}' [{}]", stringify!(#struct_name), stringify!(#field_name), stringify!(#field_numbers));
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    // println!("enum variant match for '{}::{}' ({})", stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type));
                    let value = #struct_name::#field_name(::twpb::decoder::#parse_fn(&mut bytes, stringify!(#struct_name::#field_name))?);
                    return Ok(value);
                }
            });
            let wire_type = wire_types::for_proto_type(proto_type.as_ref())
                .unwrap_or_else(|| panic!("unknown wire type for proto type '{}'", proto_type));
            encodecode.extend(quote!{
                #struct_name::#field_name(c) => {
                    bytes_written += ::twpb::encoder::tag(&mut buffer, &#first_field_number, &#wire_type)?;
                    bytes_written += ::twpb::encoder::#parse_fn(&mut buffer, c)?;
                },
            });
        }
    }

    Ok(TokenStream::from(quote!{
        impl #struct_name {
            pub fn twpb_decode<'a, I>(field_number: u32, wire_type: u8, mut bytes: I, field_name: &str) -> Result<#struct_name, ::twpb::decoder::DecodeError>
            where I: Iterator<Item = &'a u8> {
                // println!("decoding proto {}", stringify!(#struct_name));

                #debugmsg

                // println!("got field nr {}", field_number);
                // println!("got wire type {}", wire_type);

                #decodecode

                return Err(::twpb::decoder::DecodeError::UnexpectedEndOfBuffer);
            }
        }
        impl ::twpb::MessageEncoder for #struct_name {
            fn twpb_encode(&self, mut buffer: impl bytes::BufMut) -> Result<usize, ::twpb::encoder::EncodeError> {
                let mut bytes_written = 0;
                match &self {
                    #encodecode
                };
                Ok(bytes_written)
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

    let struct_name = input.ident;

    // Get all struct fields
    let fields = match input.data {
        Data::Struct(DataStruct{fields: Fields::Named(syn::FieldsNamed{named: fields, ..}), ..})
            => fields.into_iter().collect(),
        // There can also be no fields at all
        Data::Struct(DataStruct{fields: Fields::Unit, ..}) => vec![],

        Data::Struct(DataStruct{fields: Fields::Unnamed(..), ..})
            => panic!("Unnamed fields are not supported"),
        Data::Enum(..) => panic!("Message can not be derived for an enum"),
        Data::Union(..) => panic!("Message can not be derived for a union"),
    };

    // Parse each field and extract protobuf info
    let fields: Result<Vec<_>, _> = fields.into_iter()
        .map(|field| ParsedField::parse(field))
        .collect();
    let fields = fields?;

    let mut allocatecode = quote!();
    let mut decodecode = quote!();
    let mut encodecode = quote!();
    for field in fields {
        // println!("'{}::{:?}' of type {:?} has field numbers {:?}",
        //     struct_name, field.field_name, field.proto_type, field.field_numbers);

        let field_name = field.field_name;
        let proto_type = field.proto_type;
        let field_numbers = field.field_numbers.iter().map(|n| quote!(#n)).reduce(|acc, new| quote! {#acc , #new});
        let first_field_number = field.field_numbers[0];
        // allocatecode.extend(quote!{
        //     // println!("Dealing with '{}::{}' ({}) at field numbers [{}]",
        //     //     stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type), stringify!(#field_numbers));
        // });
        if field.repeated {
            allocatecode.extend(quote!{
                result.#field_name = ::heapless::Vec::new();
            });
        }

        if proto_type == "oneof" {
            // oneofs are always wrapped into a rust Option object, so we need what's _in_ the Option

            // Get the Option object
            let optionarg = match field.field_type {
                syn::Type::Path(syn::TypePath{path: syn::Path{segments: b, ..}, ..}) => 
                b.into_iter().find(|b| b.ident == "Option" && ! b.arguments.is_empty()).unwrap().arguments,
                _ => panic!("oneof field '{}' not wrapped in Option object", field_name),
            };

            // Get the arguments inside it
            let optionarg = match optionarg {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args: b, ..}) => b,
                _ => panic!("Invalid Option object for oneof field '{}'", field_name),
            };

            // There can be only one
            let optionarg = &optionarg[0];

            // println!("message encountered enum for {:?}", optionarg);
            decodecode.extend(quote!{
                if vec![#field_numbers].iter().any(|&i| i == field_number) {
                    fieldMatch = true;
                    // println!("parsing enum field of type '{}'", stringify!(#optionarg));
                    // println!("match for '{}::{}' ({})", stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type));
                    result.#field_name = #optionarg::twpb_decode(field_number, wire_type, &mut bytes, stringify!(#field_name)).ok();
                }
            });

            encodecode.extend(quote!{
                if let Some(value) = self.#field_name.as_ref() {
                    bytes_written += value.twpb_encode(&mut buffer)?;
                }
            })

        } else if proto_type == "message" {
            panic!("encountered embedded message for {}, {}", field_name, proto_type);

        } else {
            let parse_fn = Ident::new(&format!("{}", &proto_type), Span::call_site());

            let wire_type = wire_types::for_proto_type(proto_type.as_ref())
                .unwrap_or_else(|| panic!("unknown wire type for proto type '{}'", proto_type));

            // if the value is a repeated field, we need to iterate over the values
            if field.repeated {
                // Repeated fields can be encoded in packed or non-packed mode.
                // Packed repeated fields consists of length delimited tag followed by
                // all values one after another.
                // Non-packed repeated fields are just like non-repeated fields,
                // i.e. a tag and a value, except the same tag keeps getting
                // repeated for each instance
                //
                // Packed repeated fields are problematic, we need to know the size beforehand
                // which makes a streaming writer model more tricky and in need of
                // unbound temporary buffers.
                // Therefore, we will always send non-packed repeated fields.
                //
                // The spec allows this: Protocol buffer parsers must be able to parse
                // repeated fields that were compiled as packed as if they were not packed,
                // and vice versa. This permits adding [packed=true] to existing fields in a
                // forward- and backward-compatible way.
                encodecode.extend(quote!{
                    for val in self.#field_name.iter() {
                        bytes_written += ::twpb::encoder::tag(&mut buffer, &#first_field_number, &#wire_type)?;
                        bytes_written += ::twpb::encoder::#parse_fn(&mut buffer, val)?;
                    }
                });
            // non-repeated field -> just write the value
            } else {
                encodecode.extend(quote!{
                    bytes_written += ::twpb::encoder::tag(&mut buffer, &#first_field_number, &#wire_type)?;
                    bytes_written += ::twpb::encoder::#parse_fn(&mut buffer, &self.#field_name)?;
                });
            }

            if field.repeated {
                decodecode.extend(quote!{
                    if vec![#field_numbers].iter().any(|&i| i == field_number) {
                        fieldMatch = true;
                        // packed repeated field
                        // 'string' and 'bytes' are never packed, because their non-repeated encoding is already the same as packed repeated encoding
                        if wire_type == ::twpb::wire_types::LENGTHDELIMITED && #proto_type != "string" && #proto_type != "bytes" {
                            let bufsize = ::twpb::decoder::leb128_u32(&mut bytes)?;
                            let mut iterator = ::twpb::LimitedIterator::new(&mut bytes, bufsize);
                            loop {
                                match ::twpb::decoder::#parse_fn(&mut iterator, stringify!(#field_name)) {
                                    Ok(value) => result.#field_name.push(value).map_err(|_| ::twpb::decoder::DecodeError::UnexpectedEndOfBuffer)?,
                                    Err(::twpb::decoder::DecodeError::EmptyBuffer) => break,
                                    Err(e) => return Err(e),
                                };
                            }
                        // non-packed repeated field
                        } else {
                            let value = ::twpb::decoder::#parse_fn(&mut bytes, stringify!(#field_name))?;
                            result.#field_name.push(value).map_err(|_| ::twpb::decoder::DecodeError::UnexpectedEndOfBuffer)?;
                        }
                    }
                });
            } else {
                decodecode.extend(quote!{
                    if vec![#field_numbers].iter().any(|&i| i == field_number) {
                        fieldMatch = true;
                        // println!("match for '{}::{}' ({})", stringify!(#struct_name), stringify!(#field_name), stringify!(#proto_type));
                        result.#field_name = ::twpb::decoder::#parse_fn(&mut bytes, stringify!(#field_name))?;
                    }
                });
            }
        }
    }

    Ok(TokenStream::from(quote!{
        impl ::twpb::MessageDecoder for #struct_name {
            fn twpb_decode_iter<'a, I>(mut bytes: I) -> Result<#struct_name, ::twpb::decoder::DecodeError>
            where I: Iterator<Item = &'a u8> {
                // println!("decoding proto {}", stringify!(#struct_name));
                let mut result = #struct_name::default();

                #allocatecode

                // Protobuf messages are a list of key->value pairs, the key being a tag
                // which consists of the field type and the wire type.
                // As long as keys keep being encountered in the buffer, read said keys and values.
                loop {
                    match ::twpb::decoder::tag(&mut bytes) {
                        Ok((field_number, wire_type)) => {
                            // println!("got field nr {}", field_number);
                            // println!("got wire type {}", wire_type);
                            let mut fieldMatch = false;
                            #decodecode
                            if !fieldMatch {
                                ::twpb::decoder::unknown(&mut bytes, wire_type)?;
                            }
                        },
                        Err(::twpb::decoder::DecodeError::EmptyBuffer) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
        }
        impl ::twpb::MessageEncoder for #struct_name {
            fn twpb_encode(&self, mut buffer: impl bytes::BufMut) -> Result<usize, ::twpb::encoder::EncodeError> {
                let mut bytes_written = 0;
                #encodecode
                Ok(bytes_written)
            }
        }
    }))
}
