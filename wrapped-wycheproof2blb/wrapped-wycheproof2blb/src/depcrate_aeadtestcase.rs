// Generated macro for TestCase (struct)
macro_rules! Depcrate_aeadTestCase {
() => {
// Module: crate::aead
// Provides: {"TestCase"}
// Dependencies: {}
# [derive (Debug , Deserialize)] pub struct TestCase { # [serde (flatten)] pub case : wycheproof :: Case , # [serde (with = "hex_string")] pub aad : Vec < u8 > , # [serde (with = "hex_string")] pub ct : Vec < u8 > , # [serde (with = "hex_string")] pub iv : Vec < u8 > , # [serde (with = "hex_string")] pub key : Vec < u8 > , # [serde (with = "hex_string")] pub msg : Vec < u8 > , # [serde (with = "hex_string")] pub tag : Vec < u8 > , }
};
}
