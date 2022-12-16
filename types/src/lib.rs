pub mod packet {
    use std::{fmt::Display, str::FromStr};

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub enum ClientState {
        Status,
        Handshake,
        Login,
        Play,
    }

    impl FromStr for ClientState {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "Status" => Self::Status,
                "Handshake" => Self::Handshake,
                "Login" => Self::Login,
                "Play" => Self::Play,
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unable to find ClientState with name {}", s),
                ))?,
            })
        }
    }

    impl Display for ClientState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status => write!(f, "Status"),
                Self::Handshake => write!(f, "Handshake"),
                Self::Login => write!(f, "Login"),
                Self::Play => write!(f, "Play"),
            }
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub enum PacketDirection {
        Clientbound,
        Serverbound,
    }

    impl FromStr for PacketDirection {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "Clientbound" => Self::Clientbound,
                "Serverbound" => Self::Serverbound,
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unable to find PacketDirection with name {}", s),
                ))?,
            })
        }
    }

    impl Display for PacketDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Clientbound => write!(f, "Clientbound"),
                Self::Serverbound => write!(f, "Serverbound"),
            }
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub struct PacketMacroData<T> {
        pub variant: T,
        pub id: T,
        pub packet: T,
    }
}
