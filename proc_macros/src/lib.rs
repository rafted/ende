use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing, Span, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};
use types::packet::{ClientState, PacketDirection, PacketMacroData};

#[proc_macro_derive(MinecraftPacket)]
pub fn define_packet(input: TokenStream) -> TokenStream {
    // Parse the input token stream and extract the struct name and fields
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    (field_name.clone(), field_type.clone())
                })
                .collect::<Vec<(Ident, Type)>>(),
            _ => panic!("Unsupported field format for Packet derive macro"),
        },
        _ => panic!("Unsupported data format for Packet derive macro"),
    };

    let mut encode_expand = quote! {};
    let mut decode_expand = quote! {};

    let mut segments_combined = String::from("");

    for (field_name, field_type) in fields {
        if let Type::Path(path) = &field_type {
            let segments = path.path.segments.to_token_stream().to_string();
            segments_combined = segments_combined + &segments;

            if segments.contains("<") {
                let types = segments.split("<").collect::<Vec<&str>>();

                let enum_ty = types[0].parse::<proc_macro2::TokenStream>().unwrap();
                let data_ty = types[1]
                    .replace(">", "")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap();

                decode_expand.extend(quote! {
                    #field_name: #enum_ty::<#data_ty>::decode(reader)?,
                });
            } else if segments.contains("[") {
                decode_expand.extend(quote! {
                    #field_name: <#field_type as Decodable>::decode(reader)?,
                });
            } else {
                decode_expand.extend(quote! {
                    #field_name: #field_type::decode(reader)?,
                });
            }
            encode_expand.extend(quote! {
                self.#field_name.encode(writer)?;
            });
        }
    }

    // Generate the implementation of the encode and decode methods
    let expanded = quote! {
        impl crate::packets::Encodable for #name {
            fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
                #encode_expand
                Ok(())
            }
        }

        impl crate::packets::Decodable for #name {
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
                Ok(Self {
                    #decode_expand
                })
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(ParsePacket, attributes(packet))]
pub fn define_packet_parsers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    if let Data::Enum(data) = &input.data {
        let mut ids = quote! {};
        let mut id_match_map = HashMap::<
            ClientState,
            HashMap<PacketDirection, Vec<PacketMacroData<proc_macro2::TokenStream>>>,
        >::new();

        let tokens: proc_macro2::TokenStream = data
            .variants
            .iter()
            .map(|variant| {
                let attributes = &variant.attrs;
                let variant_name = &variant.ident;

                // we have to write a better parser for this, this sucks. but i can't be arsed to spend more time on this right now.
                let packet_value = attributes
                    .iter()
                    .next()
                    .unwrap()
                    .tokens
                    .to_string()
                    .replace("(", "")
                    .replace(")", "");

                let split = packet_value.split(", ").collect::<Vec<&str>>();

                let id = split[0].parse::<proc_macro2::TokenStream>().unwrap();
                let packet_direction = split[1].parse::<PacketDirection>().expect("PacketDirection was not able to be parsed from type!");
                let required_client_state = split[2].parse::<ClientState>().expect("ClientState was not able to be parsed from type!");
                let packet_value = split[3].parse::<proc_macro2::TokenStream>().unwrap();

                let packet_value_snake = to_snake_case(&packet_value.to_string())
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap();

                if !id_match_map.contains_key(&required_client_state) {
                    id_match_map.insert(required_client_state, HashMap::new());
                }

                let map = id_match_map.get_mut(&required_client_state).expect("wtf");

                if !map.contains_key(&packet_direction) {
                    map.insert(packet_direction, Vec::new());
                }

                let vec = map.get_mut(&packet_direction).expect("wha");

                let data = PacketMacroData::<proc_macro2::TokenStream> {
                    variant: variant_name.to_token_stream(),
                    id: id.clone(),
                    packet: packet_value.clone()
                };

                vec.push(data);

                ids.extend(quote! {
                    #name::#variant_name => (#id as u8),
                });

                quote! {
                    fn #packet_value_snake<R: Read>(reader: &mut R) -> Result<#packet_value, std::io::Error> {
                        #packet_value::decode(reader)
                    }
                }
            })
            .collect();

        let mut id_match_expanded = quote! {};

        let invalid_state_error = quote! {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Unimplemented or invalid packet id."))?
        };

        let ids_name = "id".parse::<proc_macro2::TokenStream>().unwrap();
        let states_name = "state".parse::<proc_macro2::TokenStream>().unwrap();
        let directions_name = "direction".parse::<proc_macro2::TokenStream>().unwrap();

        for value in id_match_map {
            let client_state = value
                .0
                .to_string()
                .parse::<proc_macro2::TokenStream>()
                .unwrap();
            let map = value.1;
            let mut directions = quote! {};

            for value in map {
                let direction = value
                    .0
                    .to_string()
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap();
                let data = value.1;
                let mut packet_ids = quote! {};

                for data in data {
                    let id = data.id;
                    // let packet_value = data.packet;
                    let variant_name = data.variant;

                    packet_ids.extend(quote! {
                        #id => Self::#variant_name,
                    });
                }

                directions.extend(quote! {
                    PacketDirection::#direction => {
                        match #ids_name {
                            #packet_ids
                            _ => #invalid_state_error
                        }
                    },
                })
            }

            id_match_expanded.extend(quote! {
                ClientState::#client_state => {
                    match #directions_name {
                        #directions
                        _ => #invalid_state_error
                    }
                },
            });
        }

        quote! {
            impl #name {
                #tokens

                fn get_id(&self) -> u8 {
                    match self {
                        #ids
                    }
                }

                fn get_from_id(
                    #ids_name: u8,
                    #states_name: ClientState,
                    #directions_name: PacketDirection
                ) -> Result<Self, std::io::Error> {
                    Ok(match #states_name {
                        #id_match_expanded
                        _ => #invalid_state_error
                    })
                }
            }
        }
        .into()
    } else {
        panic!("Only allowed on enums!");
    }
}

#[proc_macro_derive(NBTEncoder)]
pub fn define_nbt_encoder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl crate::packets::Encodable for #name {
            fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
                let tag = serialize(self)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

                writer.write_all(tag.as_slice())
            }
        }
    }
    .into()
}

#[proc_macro_derive(NBTDecoder)]
pub fn define_nbt_decoder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl crate::packets::Decodable for #name {
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
                let to_read = crate::VarInt::decode(reader)?;
                let mut buffer = vec![0; to_read as usize];

                reader.read_exact(&mut buffer)?;

                deserialize::<#name>(&buffer)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
        }
    }
    .into()
}

// we can definitely otpimize this, i just cannot be arsed to do the regex stuff.
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_upper = false;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 && !prev_is_upper {
                result.push('_');
            }
            result.extend(ch.to_lowercase());
            prev_is_upper = true;
        } else {
            result.push(ch);
            prev_is_upper = false;
        }
    }

    result
}
