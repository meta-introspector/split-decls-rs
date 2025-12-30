// Generated macro for Serialize (trait)
macro_rules! Depcrate_serSerialize {
() => {
// Module: crate::ser
// Provides: {"Serialize"}
// Dependencies: {}
# [doc = " Types that can be serialized via `#[serde(with = \"serde_bytes\")]`."] pub trait Serialize { # [allow (missing_docs)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer ; }
};
}
