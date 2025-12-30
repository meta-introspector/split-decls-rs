// Generated macro for TestGroup (struct)
macro_rules! Depcrate_aes_sivTestGroup {
() => {
// Module: crate::aes_siv
// Provides: {"TestGroup"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestGroup { # [allow (dead_code)] # [serde (flatten)] pub group : wycheproof :: Group , # [serde (rename = "keySize")] pub key_size : u32 , pub tests : Vec < TestCase > , }
};
}
