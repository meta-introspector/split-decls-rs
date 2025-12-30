// Generated macro for lit_code_hash (function)
macro_rules! Depcratelit_code_hash {
() => {
// Module: crate
// Provides: {"lit_code_hash"}
// Dependencies: {}
# [doc = " Hash a single literal from a code."] # [doc = ""] # [doc = " This doesn't require the code to correspond a valid literal."] pub fn lit_code_hash (lit_code : usize) -> ClauseHash { (! (lit_code as u64)) . wrapping_mul (0x61c8864680b583ebu64) }
};
}
