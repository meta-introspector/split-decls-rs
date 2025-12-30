// Generated macro for TestCase (struct)
macro_rules! Depcrate_ecdsaTestCase {
() => {
// Module: crate::ecdsa
// Provides: {"TestCase"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestCase { # [serde (flatten)] pub case : wycheproof :: Case , # [serde (with = "hex_string")] pub msg : Vec < u8 > , # [serde (with = "hex_string")] pub sig : Vec < u8 > , }
};
}
