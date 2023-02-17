use std::fmt::Display;

use crate::messages::message_types::message_constants::MessageId;

use crate::message_traits::serializable::Serializable;

pub struct HumanMessage {
    first_name_initial: u8,
    last_name_initial: u8,
    age: u8,
    income: i64,
}

impl Serializable for HumanMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut return_bytes = vec![MessageId::Human.to_u8()];
        let first_name_initial_byte = self.first_name_initial.to_ne_bytes();
        for i in first_name_initial_byte.iter() {
            return_bytes.push(*i);
        }
        let last_name_initial_byte = self.last_name_initial.to_ne_bytes();
        for i in last_name_initial_byte.iter() {
            return_bytes.push(*i);
        }
        let age_byte = self.age.to_ne_bytes();
        for i in age_byte.iter() {
            return_bytes.push(*i);
        }
        let income_bytes = self.income.to_ne_bytes();
        for i in income_bytes.iter() {
            return_bytes.push(*i);
        }
        return_bytes
    }

    fn deserialize(bytes: Vec<u8>) -> Self {
        let first_name_initial = bytes[0];
        let last_name_initial = bytes[1];
        let age = bytes[2];
        let income = i64::from_ne_bytes(bytes[3..=10].try_into().unwrap());
        HumanMessage {
            first_name_initial,
            last_name_initial,
            age,
            income,
        }
    }
}

impl Display for HumanMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. {}. is {} years old and earns {} per year",
            self.first_name_initial as char, self.last_name_initial as char, self.age, self.income
        )
    }
}
