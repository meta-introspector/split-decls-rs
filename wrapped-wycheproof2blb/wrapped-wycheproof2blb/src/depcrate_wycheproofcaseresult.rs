// Generated macro for CaseResult (enum)
macro_rules! Depcrate_wycheproofCaseResult {
() => {
// Module: crate::wycheproof
// Provides: {"CaseResult"}
// Dependencies: {}
# [doc = " `Result` represents the possible result values for a Wycheproof test case."] # [derive (Debug , PartialEq , Eq)] pub enum CaseResult { # [doc = " Test case is valid, the crypto operation should succeed."] Valid , # [doc = " Test case is invalid; the crypto operation should fail."] Invalid , # [doc = " Test case is valid, but uses weak parameters; the crypto operation might succeed"] # [doc = " or fail depending on how strict the library is."] Acceptable , }
};
}
