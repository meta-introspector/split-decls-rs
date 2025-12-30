// Generated macro for TestSuite (struct)
macro_rules! Depcrate_ed25519TestSuite {
() => {
// Module: crate::ed25519
// Provides: {"TestSuite"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct TestSuite { # [serde (flatten)] pub suite : wycheproof :: Suite , # [serde (rename = "testGroups")] pub test_groups : Vec < TestGroup > , }
};
}
