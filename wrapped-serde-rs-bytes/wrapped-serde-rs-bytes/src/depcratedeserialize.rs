// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Serde `deserialize_with` function to deserialize bytes efficiently."] # [doc = ""] # [doc = " This function can be used with either of the following Serde attributes:"] # [doc = ""] # [doc = " - `#[serde(with = \"serde_bytes\")]`"] # [doc = " - `#[serde(deserialize_with = \"serde_bytes::deserialize\")]`"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_derive::Deserialize;"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Packet {"] # [doc = "     #[serde(with = \"serde_bytes\")]"] # [doc = "     payload: Vec<u8>,"] # [doc = ""] # [doc = "     #[serde(with = \"serde_bytes\")]"] # [doc = "     byte_array: [u8; 314],"] # [doc = " }"] # [doc = " ```"] pub fn deserialize < 'de , T , D > (deserializer : D) -> Result < T , D :: Error > where T : Deserialize < 'de > , D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) }
};
}
