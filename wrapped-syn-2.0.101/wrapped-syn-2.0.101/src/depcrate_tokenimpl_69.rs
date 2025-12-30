// Generated macro for impl_69 (impl)
macro_rules! Depcrate_tokenimpl_69 {
() => {
// Module: crate::token
// Provides: {"impl_69"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Token for Underscore { fn peek (cursor : Cursor) -> bool { if let Some ((ident , _rest)) = cursor . ident () { return ident == "_" ; } if let Some ((punct , _rest)) = cursor . punct () { return punct . as_char () == '_' ; } false } fn display () -> & 'static str { "`_`" } }
};
}
