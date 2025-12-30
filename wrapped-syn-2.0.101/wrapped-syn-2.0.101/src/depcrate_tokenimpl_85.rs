// Generated macro for impl_85 (impl)
macro_rules! Depcrate_tokenimpl_85 {
() => {
// Module: crate::token
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Token for Group { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: None) . is_some () } fn display () -> & 'static str { "invisible group" } }
};
}
