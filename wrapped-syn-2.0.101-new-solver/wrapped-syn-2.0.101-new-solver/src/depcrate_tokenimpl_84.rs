// Generated macro for impl_84 (impl)
macro_rules! Depcrate_tokenimpl_84 {
() => {
// Module: crate::token
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Token for Bracket { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: Bracket) . is_some () } fn display () -> & 'static str { "square brackets" } }
};
}
