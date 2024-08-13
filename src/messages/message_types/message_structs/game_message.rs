use std::fmt::Display;

use crate::message_traits::serializable::Serializable;

use super::super::message_id::MessageId;

/// Message representing a game that can be serialized
#[derive(Clone)]
pub struct GameMessage {
    rating: f32,
    price: f32,
    active_players: u64,
}

impl GameMessage {
    /// Generate a message from the fields
    pub fn new(rating: f32, price: f32, active_players: u64) -> Self {
        GameMessage {
            rating,
            price,
            active_players,
        }
    }
}

impl Serializable for GameMessage {
    /// Serialize the message
    fn serialize(&self) -> Vec<u8> {
        let mut return_bytes = vec![MessageId::Game.into()];
        let rating_bytes = self.rating.to_ne_bytes();
        for i in rating_bytes.iter() {
            return_bytes.push(*i);
        }
        let price_bytes = self.price.to_ne_bytes();
        for i in price_bytes.iter() {
            return_bytes.push(*i);
        }
        let active_players_bytes = self.active_players.to_ne_bytes();
        for i in active_players_bytes.iter() {
            return_bytes.push(*i);
        }
        return_bytes
    }

    /// Deserialize the message
    fn deserialize(bytes: Vec<u8>) -> Self {
        let rating = f32::from_ne_bytes(bytes[0..=3].try_into().unwrap());
        let price = f32::from_ne_bytes(bytes[4..=7].try_into().unwrap());
        let active_players = u64::from_ne_bytes(bytes[8..=15].try_into().unwrap());
        GameMessage {
            rating,
            price,
            active_players,
        }
    }
}

impl Display for GameMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "This game is rated as {}, costs {}, and has {} active players",
            self.rating, self.price, self.active_players
        )
    }
}
