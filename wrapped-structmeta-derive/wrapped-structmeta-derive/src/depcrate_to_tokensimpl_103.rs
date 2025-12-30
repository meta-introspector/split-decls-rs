// Generated macro for impl_103 (impl)
macro_rules! Depcrate_to_tokensimpl_103 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_103"}
// Dependencies: {}
impl Surround < '_ > { fn token_type_ident (& self) -> Ident { match self . delimiter { Delimiter :: Bracket => parse_quote ! (Bracket) , Delimiter :: Brace => parse_quote ! (Brace) , Delimiter :: Parenthesis => parse_quote ! (Paren) , _ => unreachable ! ("unsupported delimiter") , } } }
};
}
