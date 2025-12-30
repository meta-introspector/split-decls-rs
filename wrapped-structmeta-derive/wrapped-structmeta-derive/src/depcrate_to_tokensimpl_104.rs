// Generated macro for impl_104 (impl)
macro_rules! Depcrate_to_tokensimpl_104 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_104"}
// Dependencies: {}
impl Scope < '_ > { fn into_code (self , delimiter : Option < Delimiter > , span : Span) -> Result < TokenStream > { if let Some (s) = self . surround { if let Some (delimiter) = delimiter { if s . delimiter != delimiter { bail ! (span , "mismatched closing delimiter expected `{}`, found `{}`." , close_char_of (s . delimiter) , close_char_of (delimiter) ,) } } let ident = & s . ident ; let ts = self . ts ; let ty = & s . field . ty ; let span = s . field . span () ; let func = if is_macro_delimiter (ty) { quote_spanned ! (span => :: structmeta :: helpers :: surround_macro_delimiter) } else { let ty = s . token_type_ident () ; quote_spanned ! (span => :: structmeta :: helpers :: exports :: syn :: token ::# ty :: surround) } ; let code = quote_spanned ! (span => # func (# ident , tokens , | tokens | { # ts }) ;) ; return Ok (code) ; } Ok (quote ! ()) } }
};
}
