// Generated macro for impl_82 (impl)
macro_rules! Depcrate_tokenimpl_82 {
() => {
// Module: crate::token
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Token for Paren { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: Parenthesis) . is_some () } fn display () -> & 'static str { "parentheses" } }
};
}
