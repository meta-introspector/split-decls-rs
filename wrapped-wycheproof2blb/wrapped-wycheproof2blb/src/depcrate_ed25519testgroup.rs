// Generated macro for TestGroup (struct)
macro_rules! Depcrate_ed25519TestGroup {
() => {
// Module: crate::ed25519
// Provides: {"TestGroup"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestGroup { # [allow (dead_code)] # [serde (flatten)] pub group : wycheproof :: Group , # [allow (dead_code)] # [serde (rename = "keyDer")] pub key_der : String , # [allow (dead_code)] # [serde (rename = "keyPem")] pub key_pem : String , pub key : TestKey , pub tests : Vec < TestCase > , }
};
}
