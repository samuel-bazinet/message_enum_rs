/// Trait used to make structs serializable
pub trait Serializable {
    /// Serialize the struct into bytes
    fn serialize(&self) -> Vec<u8>;
    /// Deserialize bytes into a struct
    fn deserialize(bytes: Vec<u8>) -> Self;
}
