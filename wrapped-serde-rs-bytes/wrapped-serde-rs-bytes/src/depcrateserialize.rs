// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serde `serialize_with` function to serialize bytes efficiently."] # [doc = ""] # [doc = " This function can be used with either of the following Serde attributes:"] # [doc = ""] # [doc = " - `#[serde(with = \"serde_bytes\")]`"] # [doc = " - `#[serde(serialize_with = \"serde_bytes::serialize\")]`"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_derive::Serialize;"] # [doc = " use serde::Serialize;"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Efficient<'a> {"] # [doc = "     #[serde(with = \"serde_bytes\")]"] # [doc = "     bytes: &'a [u8],"] # [doc = ""] # [doc = "     #[serde(with = \"serde_bytes\")]"] # [doc = "     byte_buf: Vec<u8>,"] # [doc = ""] # [doc = "     #[serde(with = \"serde_bytes\")]"] # [doc = "     byte_array: [u8; 314],"] # [doc = " }"] # [doc = " ```"] pub fn serialize < T , S > (bytes : & T , serializer : S) -> Result < S :: Ok , S :: Error > where T : ? Sized + Serialize , S : Serializer , { Serialize :: serialize (bytes , serializer) }
};
}
