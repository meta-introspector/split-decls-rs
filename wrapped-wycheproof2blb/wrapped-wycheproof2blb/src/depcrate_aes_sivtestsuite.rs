// Generated macro for TestSuite (struct)
macro_rules! Depcrate_aes_sivTestSuite {
() => {
// Module: crate::aes_siv
// Provides: {"TestSuite"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestSuite { # [serde (flatten)] pub suite : wycheproof :: Suite , # [serde (rename = "testGroups")] pub test_groups : Vec < TestGroup > , }
};
}
