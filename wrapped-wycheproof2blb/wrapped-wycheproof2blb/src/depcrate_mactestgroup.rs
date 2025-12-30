// Generated macro for TestGroup (struct)
macro_rules! Depcrate_macTestGroup {
() => {
// Module: crate::mac
// Provides: {"TestGroup"}
// Dependencies: {}
# [derive (Debug , Deserialize)] pub struct TestGroup { # [serde (flatten)] pub group : wycheproof :: Group , # [serde (rename = "keySize")] pub key_size : u32 , # [serde (rename = "tagSize")] pub tag_size : u32 , pub tests : Vec < TestCase > , }
};
}
