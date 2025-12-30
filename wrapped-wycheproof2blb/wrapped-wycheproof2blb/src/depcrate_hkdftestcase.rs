// Generated macro for TestCase (struct)
macro_rules! Depcrate_hkdfTestCase {
() => {
// Module: crate::hkdf
// Provides: {"TestCase"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestCase { # [serde (flatten)] pub case : wycheproof :: Case , # [serde (with = "hex_string")] pub ikm : Vec < u8 > , # [serde (with = "hex_string")] pub salt : Vec < u8 > , # [serde (with = "hex_string")] pub info : Vec < u8 > , pub size : usize , # [serde (with = "hex_string")] pub okm : Vec < u8 > , }
};
}
