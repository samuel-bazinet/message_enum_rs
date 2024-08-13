#[derive(Clone)]
#[repr(u8)]
pub enum MessageId {
    Human = 0,
    Pet,
    Game,
}

impl From<u8> for MessageId {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Human,
            1 => Self::Pet,
            2 => Self::Game,
            _ => panic!("Invalid ID provided"),
        }
    }
}

impl From<MessageId> for u8 {
    fn from(value: MessageId) -> Self {
        value as u8
    }
}
