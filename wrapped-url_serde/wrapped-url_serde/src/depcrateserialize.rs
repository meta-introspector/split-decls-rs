// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialises `value` with a given serializer."] # [doc = ""] # [doc = " This is useful to serialize `rust-url` types used in structure fields or"] # [doc = " tuple members with `#[serde(serialize_with = \"url_serde::serialize\")]`."] pub fn serialize < T , S > (value : & T , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , for < 'a > Ser < 'a , T > : Serialize { Ser :: new (value) . serialize (serializer) }
};
}
