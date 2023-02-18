#[derive(Clone)]
#[repr(u8)]
pub enum MessageId {
    Human = 0,
    Pet,
    Game,
}

impl MessageId {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Human,
            1 => Self::Pet,
            2 => Self::Game,
            _ => panic!("Invalid ID provided"),
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.clone() as u8
    }
}
