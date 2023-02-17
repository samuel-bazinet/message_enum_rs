pub mod pet_class;

use crate::messages::message_types::message_constants::MessageId;
use pet_class::PetClass;

use crate::message_traits::serializable::Serializable;

use std::fmt::Display;

pub struct PetMessage {
    initial: u8,
    class: PetClass,
    age: u8,
}

impl Serializable for PetMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut return_bytes = vec![MessageId::Pet.to_u8()];
        let initial_bytes = self.initial.to_ne_bytes();
        for i in initial_bytes.iter() {
            return_bytes.push(*i);
        }
        let class_bytes = (self.class as u8).to_ne_bytes();
        for i in class_bytes.iter() {
            return_bytes.push(*i);
        }
        let age_bytes = self.age.to_ne_bytes();
        for i in age_bytes.iter() {
            return_bytes.push(*i);
        }
        return_bytes
    }

    fn deserialize(bytes: Vec<u8>) -> Self {
        let initial = bytes[0];
        let class = PetClass::try_from(bytes[1]).unwrap();
        let age = bytes[2];
        PetMessage {
            initial,
            class,
            age,
        }
    }
}

impl Display for PetMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The pet {} is a {} and is {} years old",
            self.initial as char, self.class, self.age
        )
    }
}
