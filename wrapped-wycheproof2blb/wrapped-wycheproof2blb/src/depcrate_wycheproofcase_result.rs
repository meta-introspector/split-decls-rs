// Generated macro for case_result (function)
macro_rules! Depcrate_wycheproofcase_result {
() => {
// Module: crate::wycheproof
// Provides: {"case_result"}
// Dependencies: {}
# [doc = " Convert a `result` enum to a byte."] pub fn case_result (case : & Case) -> u8 { match case . result { CaseResult :: Invalid => 0u8 , CaseResult :: Valid => 1u8 , _ => panic ! ("Unexpected case result {}" , case . result) , } }
};
}
