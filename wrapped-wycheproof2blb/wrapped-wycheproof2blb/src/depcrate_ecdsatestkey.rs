// Generated macro for TestKey (struct)
macro_rules! Depcrate_ecdsaTestKey {
() => {
// Module: crate::ecdsa
// Provides: {"TestKey"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestKey { curve : String , # [allow (dead_code)] # [serde (rename = "type")] key_type : String , # [serde (with = "hex_string")] wx : Vec < u8 > , # [serde (with = "hex_string")] wy : Vec < u8 > , }
};
}
