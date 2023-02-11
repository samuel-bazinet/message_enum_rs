use std::fmt::Display;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PetClass {
    PureBreed,
    Mutt
}

impl TryFrom<u8> for PetClass {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PetClass::PureBreed),
            1 => Ok(PetClass::Mutt),
            _ => Err("Invalid value")
        }
    }
}

impl Display for PetClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PureBreed => write!(f, "pure breed"),
            Self::Mutt => write!(f, "mutt")
        }
    }
}