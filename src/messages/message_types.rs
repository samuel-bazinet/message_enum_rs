pub mod message_constants;
pub mod message_structs;

use std::fmt::Display;

use message_structs::{
    game_message::GameMessage, human_message::HumanMessage,
    message_traits::serializable::Serializable, pet_message::PetMessage,
};

use message_constants::MessageId;

pub enum MessageTypes {
    HumanMessageType(HumanMessage),
    PetMessageType(PetMessage),
    GameMessageType(GameMessage),
}

impl MessageTypes {
    fn from_bytes(mut bytes: Vec<u8>) -> Self {
        let id_byte = bytes.remove(0);
        match MessageId::from_u8(id_byte) {
            MessageId::Human => Self::HumanMessageType(HumanMessage::deserialize(bytes)),
            MessageId::Pet => Self::PetMessageType(PetMessage::deserialize(bytes)),
            MessageId::Game => Self::GameMessageType(GameMessage::deserialize(bytes)),
        }
    }
}

impl Display for MessageTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GameMessageType(a) => write!(f, "{a}"),
            Self::HumanMessageType(a) => write!(f, "{a}"),
            Self::PetMessageType(a) => write!(f, "{a}"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_bytes() {
        let pet_bytes = vec![1_u8, 76, 1, 4];

        let message_enum = MessageTypes::from_bytes(pet_bytes);

        println!("{}", message_enum);
    }
}
