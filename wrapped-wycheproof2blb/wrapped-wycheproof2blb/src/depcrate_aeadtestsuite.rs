// Generated macro for TestSuite (struct)
macro_rules! Depcrate_aeadTestSuite {
() => {
// Module: crate::aead
// Provides: {"TestSuite"}
// Dependencies: {}
# [derive (Debug , Deserialize)] pub struct TestSuite { # [serde (flatten)] pub suite : wycheproof :: Suite , # [serde (rename = "testGroups")] pub test_groups : Vec < TestGroup > , }
};
}
