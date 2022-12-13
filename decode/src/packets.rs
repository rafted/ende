pub mod login {
    use crate::decoding::Decodable;
    use crate::encoding::Encodable;
    use proc_macros::MinecraftPacket;
    use std::io::{Read, Write};
    use uuid::Uuid;

    #[derive(MinecraftPacket, Debug)]
    pub struct LoginRequest {
        pub id: u8,
        pub uuid: Uuid,
        pub username: String,
    }
}
