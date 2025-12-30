// Generated macro for impl_866 (impl)
macro_rules! Depcrate_macrosimpl_866 {
() => {
// Module: crate::macros
// Provides: {"impl_866"}
// Dependencies: {}
impl < 'a > MacroParser < 'a > { const fn new (iter : TokenStreamIter < 'a >) -> Self { Self { iter } } fn parse (& mut self) -> Option < Macro > { let mut branches = vec ! [] ; while self . iter . peek () . is_some () { branches . push (self . parse_branch () ?) ; } Some (Macro { branches }) } fn parse_branch (& mut self) -> Option < MacroBranch > { let tok = self . iter . next () ? ; let (lo , args_paren_kind) = match tok { TokenTree :: Token (..) => return None , & TokenTree :: Delimited (delimited_span , _ , d , _) => (delimited_span . open . lo () , d) , } ; let args = TokenStream :: new (vec ! [tok . clone ()]) ; match self . iter . next () ? { TokenTree :: Token (Token { kind : TokenKind :: FatArrow , .. } , _ ,) => { } _ => return None , } let (mut hi , body , whole_body) = match self . iter . next () ? { TokenTree :: Token (..) => return None , TokenTree :: Delimited (delimited_span , ..) => { let data = delimited_span . entire () . data () ; (data . hi , Span :: new (data . lo + BytePos (1) , data . hi - BytePos (1) , data . ctxt , data . parent ,) , delimited_span . entire () ,) } } ; if let Some (TokenTree :: Token (Token { kind : TokenKind :: Semi , span , } , _ ,)) = self . iter . peek () { hi = span . hi () ; self . iter . next () ; } Some (MacroBranch { span : mk_sp (lo , hi) , args_paren_kind , args , body , whole_body , }) } }
};
}
