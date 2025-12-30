// Generated macro for next_space (function)
macro_rules! Depcrate_macrosnext_space {
() => {
// Module: crate::macros
// Provides: {"next_space"}
// Dependencies: {}
fn next_space (tok : & TokenKind) -> SpaceState { debug ! ("next_space: {:?}" , tok) ; match tok { TokenKind :: Bang | TokenKind :: And | TokenKind :: Tilde | TokenKind :: At | TokenKind :: Comma | TokenKind :: Dot | TokenKind :: DotDot | TokenKind :: DotDotDot | TokenKind :: DotDotEq | TokenKind :: Question => SpaceState :: Punctuation , TokenKind :: PathSep | TokenKind :: Pound | TokenKind :: Dollar | TokenKind :: OpenDelim (_) | TokenKind :: CloseDelim (_) => SpaceState :: Never , TokenKind :: Literal (..) | TokenKind :: Ident (..) | TokenKind :: Lifetime (..) => { SpaceState :: Ident } _ => SpaceState :: Always , } }
};
}
