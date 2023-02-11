mod message_structs;

use message_structs::{
    human_message::HumanMessage,
    pet_message::PetMessage,
};

enum MessageTypes {
    HumanMessageType(HumanMessage),
    PetMessageType(PetMessage)
}