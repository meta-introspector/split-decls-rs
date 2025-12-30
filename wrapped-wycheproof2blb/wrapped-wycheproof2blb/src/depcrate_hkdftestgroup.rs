// Generated macro for TestGroup (struct)
macro_rules! Depcrate_hkdfTestGroup {
() => {
// Module: crate::hkdf
// Provides: {"TestGroup"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestGroup { # [allow (dead_code)] # [serde (flatten)] pub group : wycheproof :: Group , # [allow (dead_code)] # [serde (rename = "keySize")] pub key_size : u32 , pub tests : Vec < TestCase > , }
};
}
