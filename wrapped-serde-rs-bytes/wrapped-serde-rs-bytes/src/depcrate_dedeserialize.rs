// Generated macro for Deserialize (trait)
macro_rules! Depcrate_deDeserialize {
() => {
// Module: crate::de
// Provides: {"Deserialize"}
// Dependencies: {}
# [doc = " Types that can be deserialized via `#[serde(with = \"serde_bytes\")]`."] pub trait Deserialize < 'de > : Sized { # [allow (missing_docs)] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > ; }
};
}
