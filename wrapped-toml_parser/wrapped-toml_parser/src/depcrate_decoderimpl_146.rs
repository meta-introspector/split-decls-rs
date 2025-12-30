// Generated macro for impl_146 (impl)
macro_rules! Depcrate_decoderimpl_146 {
() => {
// Module: crate::decoder
// Provides: {"impl_146"}
// Dependencies: {}
impl Encoding { pub const fn description (& self) -> & 'static str { match self { Self :: LiteralString => crate :: lexer :: TokenKind :: LiteralString . description () , Self :: BasicString => crate :: lexer :: TokenKind :: BasicString . description () , Self :: MlLiteralString => crate :: lexer :: TokenKind :: MlLiteralString . description () , Self :: MlBasicString => crate :: lexer :: TokenKind :: MlBasicString . description () , } } }
};
}
