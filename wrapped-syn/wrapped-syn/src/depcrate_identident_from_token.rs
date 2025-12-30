// Generated macro for ident_from_token (macro)
macro_rules! Depcrate_identident_from_token {
() => {
// Module: crate::ident
// Provides: {"ident_from_token"}
// Dependencies: {}
macro_rules ! ident_from_token { ($ token : ident) => { impl From < Token ! [$ token] > for Ident { fn from (token : Token ! [$ token]) -> Ident { Ident :: new (stringify ! ($ token) , token . span) } } } ; }
};
}
