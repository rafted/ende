use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

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

    for (field_name, field_type) in fields {
        encode_expand.extend(quote! {
            self.#field_name.encode(writer)?;
        });

        decode_expand.extend(quote! {
            #field_name: #field_type::decode(reader)?,
        });
    }

    // Generate the implementation of the encode and decode methods
    let expanded = quote! {
        impl #name {
            pub fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
                #encode_expand
                Ok(())
            }

            pub fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
                Ok(Self {
                    #decode_expand
                })
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(ParsePacket)]
pub fn define_packet_parsers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    if let Data::Enum(data) = &input.data {
        let tokens = data
            .variants
            .iter()
            .map(|variant| {
                let variant_name = &variant.ident;
                let variant_data = &variant.fields.iter().next().as_ref().unwrap().ty;

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

                let snake_variant_name = to_snake_case(&variant_name.to_string())
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap();

                if let syn::Type::Path(variant_data) = variant_data {
                    let path = &variant_data.path;
                    if let Some(segment) = path.segments.last() {
                        let type_name = &segment.ident;

                        quote! {
                            fn #snake_variant_name<R: Read>(reader: &mut R) -> Result<#name, std::io::Error> {
                                Ok(#name::#variant_name(#type_name::decode(reader)?))
                            }
                        }
                    } else {
                        panic!("Invalid variant data!")
                    }
                }  else {
                    panic!("Invalid variant data!")
                }

            })
            .map(TokenStream::from)
            .collect::<TokenStream>();

        let tokens: proc_macro2::TokenStream = tokens.into();


        return quote! {
            impl #name {
                #tokens
            }
        }
        .into();
    } else {
        panic!("Only allowed on enums!");
    }
}

#[proc_macro_derive(NBTEncoder)]
pub fn define_nbt_encoder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl Encodable for #name {
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
        impl Decodable for #name {
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
                let to_read = VarInt::decode(reader)?;
                let mut buffer = vec![0; to_read as usize];

                reader.read_exact(&mut buffer)?;

                deserialize::<#name>(&buffer)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
        }
    }
    .into()
}
