// Generated macro for TestKey (struct)
macro_rules! Depcrate_ed25519TestKey {
() => {
// Module: crate::ed25519
// Provides: {"TestKey"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestKey { # [serde (with = "hex_string")] sk : Vec < u8 > , # [serde (with = "hex_string")] pk : Vec < u8 > , }
};
}
