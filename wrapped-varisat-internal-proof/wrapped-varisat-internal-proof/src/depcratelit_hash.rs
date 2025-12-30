// Generated macro for lit_hash (function)
macro_rules! Depcratelit_hash {
() => {
// Module: crate
// Provides: {"lit_hash"}
// Dependencies: {}
# [doc = " Hash a single literal."] # [doc = ""] # [doc = " Multiple literals can be combined with xor, as done in [`clause_hash`]."] pub fn lit_hash (lit : Lit) -> ClauseHash { lit_code_hash (lit . code ()) }
};
}
