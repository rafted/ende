pub mod decoding;
pub mod encoding;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod proc_test {
    use crate::{decoding::Decodable, encoding::Encodable};
    use proc_macros::*;
    use uuid::Uuid;
    use std::io::{Read, Write, Cursor};

    #[derive(MinecraftPacket, Debug)]
    struct LoginRequest {
        id: u8,
        uuid: Uuid,
        username: String,
    }

    #[test]
    fn test_login_request() {
        let request = LoginRequest {
            id: 3,
            uuid: Uuid::new_v4(),
            username: String::from("wasd"),
        };

        let buf = &mut Vec::<u8>::new();
        request.encode(buf).unwrap();

        let cursor = &mut Cursor::new(&buf);
        println!("{:?}, {:?}", buf, LoginRequest::decode(cursor));
    }
}
