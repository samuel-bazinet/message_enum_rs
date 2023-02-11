use std::fmt::Display;

use super::message_traits::serializable::Serializable;

use super::super::message_constants::MessageId;

pub struct GameMessage {
    rating: f32,
    price: f32,
    active_players: u64,
}

impl Serializable for GameMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut return_bytes = vec![MessageId::Game.to_u8()];
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
