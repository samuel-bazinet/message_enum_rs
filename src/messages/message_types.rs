mod message_id;
pub mod message_structs;

use std::fmt::Display;

use crate::message_traits::serializable::Serializable;
use message_structs::{
    game_message::GameMessage, human_message::HumanMessage, pet_message::PetMessage,
};

use message_id::MessageId;

#[derive(Clone)]
pub enum MessageTypes {
    PetMessageType(PetMessage),
    HumanMessageType(HumanMessage),
    GameMessageType(GameMessage),
}

impl From<Vec<u8>> for MessageTypes {
    fn from(mut value: Vec<u8>) -> Self {
        let id_byte = value.remove(0);
        match id_byte.into() {
            MessageId::Human => Self::HumanMessageType(HumanMessage::deserialize(value)),
            MessageId::Pet => Self::PetMessageType(PetMessage::deserialize(value)),
            MessageId::Game => Self::GameMessageType(GameMessage::deserialize(value)),
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

        let message_enum = MessageTypes::from(pet_bytes);

        println!("{}", message_enum);
    }
}
