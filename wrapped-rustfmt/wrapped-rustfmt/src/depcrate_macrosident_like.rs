// Generated macro for ident_like (function)
macro_rules! Depcrate_macrosident_like {
() => {
// Module: crate::macros
// Provides: {"ident_like"}
// Dependencies: {}
fn ident_like (tok : & Token) -> bool { matches ! (tok . kind , TokenKind :: Ident (..) | TokenKind :: Literal (..) | TokenKind :: Lifetime (..)) }
};
}
