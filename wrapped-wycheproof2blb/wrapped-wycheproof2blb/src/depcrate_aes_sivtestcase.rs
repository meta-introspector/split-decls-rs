// Generated macro for TestCase (struct)
macro_rules! Depcrate_aes_sivTestCase {
() => {
// Module: crate::aes_siv
// Provides: {"TestCase"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestCase { # [serde (flatten)] pub case : wycheproof :: Case , # [serde (with = "hex_string")] pub key : Vec < u8 > , # [serde (with = "hex_string")] pub aad : Vec < u8 > , # [serde (with = "hex_string")] pub msg : Vec < u8 > , # [serde (with = "hex_string")] pub ct : Vec < u8 > , }
};
}
