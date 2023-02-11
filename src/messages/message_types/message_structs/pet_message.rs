pub mod pet_class;

use pet_class::PetClass;

pub struct PetMessage {
    initial: u8,
    class: PetClass,
    age: u8
}