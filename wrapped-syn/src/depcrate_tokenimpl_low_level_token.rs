// Generated macro for impl_low_level_token (macro)
macro_rules! Depcrate_tokenimpl_low_level_token {
() => {
// Module: crate::token
// Provides: {"impl_low_level_token"}
// Dependencies: {}
macro_rules ! impl_low_level_token { ($ display : literal $ ($ path : ident) ::+ $ get : ident) => { # [cfg (feature = "parsing")] impl Token for $ ($ path) ::+ { fn peek (cursor : Cursor) -> bool { cursor .$ get () . is_some () } fn display () -> &'static str { $ display } } # [cfg (feature = "parsing")] impl private :: Sealed for $ ($ path) ::+ { } } ; }
};
}
