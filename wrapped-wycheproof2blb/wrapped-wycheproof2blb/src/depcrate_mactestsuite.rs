// Generated macro for TestSuite (struct)
macro_rules! Depcrate_macTestSuite {
() => {
// Module: crate::mac
// Provides: {"TestSuite"}
// Dependencies: {}
# [derive (Debug , Deserialize)] pub struct TestSuite { # [serde (flatten)] pub suite : wycheproof :: Suite , # [serde (rename = "testGroups")] pub test_groups : Vec < TestGroup > , }
};
}
