// Generated macro for impl_83 (impl)
macro_rules! Depcrate_tokenimpl_83 {
() => {
// Module: crate::token
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Token for Brace { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: Brace) . is_some () } fn display () -> & 'static str { "curly braces" } }
};
}
