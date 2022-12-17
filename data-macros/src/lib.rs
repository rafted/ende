use std::{collections::HashMap, convert, io::Result};

use minecraft_data_rs::{
    models::{
        protocol::{types::TypeName, NativeType, PacketDataType},
        version::Version,
    },
    Api,
};
use proc_macro::TokenStream;
use quote::quote;
use types::packet::{ClientState, PacketDirection};

#[proc_macro_derive(PacketDefine)]
pub fn define_packets(input: TokenStream) -> TokenStream {
    let apis = get_api_versions();

    for api in apis {
        let protocol = api
            .protocols
            .get_protocol()
            .expect("Unable to get protocol from version!");

        for packet_type in [
            protocol.handshaking.to_client,
            protocol.handshaking.to_server,
        ] {
            let mapper = packet_type.packet_mapper;
            let switch = mapper.switch;
            let mapper = mapper.mapper;

            for ty in packet_type.types {
                let name = ty.name;
                let data = ty.data;

                if let PacketDataType::Built { name: _, value } = data {
                    if let NativeType::Container(value) = &value {
                        let process_map = |field: &(TypeName, Box<PacketDataType>)| -> Option<proc_macro2::TokenStream> {
                            let name = match &field.0 {
                                TypeName::Named(name) => name,
                                TypeName::Anonymous => None?,
                            };

                            let mut formatted_name = to_snake_case(name);

                            if formatted_name == "type" {
                                formatted_name = String::from("ty");
                            }

                            let name = formatted_name
                                .parse::<proc_macro2::TokenStream>()
                                .ok()?;
                            let ty = convert_type(&*field.1)
                                .ok()?
                                .parse::<proc_macro2::TokenStream>()
                                .ok()?;

                            Some(quote! {
                                pub #name: #ty,
                            })
                        };

                        let fields = value
                            .iter()
                            .map(process_map)
                            .flatten()
                            .collect::<proc_macro2::TokenStream>();

                        // we still have to do something with this. this is the actual packet!
                        quote! {
                            #[derive(proc_macros::MinecraftPacket, Debug, PartialEq)]
                            pub struct #name {
                                #fields
                            }
                        };
                    }
                }
            }
        }
    }

    input
}

fn convert_type(ty: &PacketDataType) -> Result<String> {
    return Ok(match ty {
        PacketDataType::Native(native) => match native {
            NativeType::U8
            | NativeType::U16
            | NativeType::U32
            | NativeType::U64
            | NativeType::I8
            | NativeType::I16
            | NativeType::I32
            | NativeType::I64
            | NativeType::F32
            | NativeType::F64 => native.to_string().to_lowercase(),
            NativeType::Option(second) => format!("Option<{}>", convert_type(second)?),
            NativeType::Uuid => String::from("uuid::Uuid"),
            _ => todo!(""),
        },
        _ => todo!(""),
    });
}

fn get_api_versions() -> Vec<Api> {
    vec![
        Api::latest().expect("Unable to retrieve latest version!"),
        Api::new(Version {
            version: 47,
            minecraft_version: String::from("1.8.9"),
            major_version: String::from("1.8"),
        }),
    ]
}

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

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_underscore = false;

    for ch in s.chars() {
        if ch == '_' {
            prev_is_underscore = true;
        } else {
            if prev_is_underscore {
                result.extend(ch.to_uppercase());
            } else {
                result.push(ch);
            }
            prev_is_underscore = false;
        }
    }

    result
}
