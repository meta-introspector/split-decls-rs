// Generated macro for TestGroup (struct)
macro_rules! Depcrate_aeadTestGroup {
() => {
// Module: crate::aead
// Provides: {"TestGroup"}
// Dependencies: {}
# [derive (Debug , Deserialize)] pub struct TestGroup { # [serde (flatten)] pub group : wycheproof :: Group , # [serde (rename = "ivSize")] pub iv_size : u32 , # [serde (rename = "keySize")] pub key_size : u32 , # [serde (rename = "tagSize")] pub tag_size : u32 , pub tests : Vec < TestCase > , }
};
}
