// Generated macro for impl_257 (impl)
macro_rules! Depcrate_errorimpl_257 {
() => {
// Module: crate::error
// Provides: {"impl_257"}
// Dependencies: {}
impl ErrorMessage { fn to_compile_error (& self , tokens : & mut TokenStream) { let (start , end) = match self . span . get () { Some (range) => (range . start , range . end) , None => (Span :: call_site () , Span :: call_site ()) , } ; tokens . append (TokenTree :: Punct (Punct :: new_spanned (':' , Spacing :: Joint , start ,))) ; tokens . append (TokenTree :: Punct (Punct :: new_spanned (':' , Spacing :: Alone , start ,))) ; tokens . append (TokenTree :: Ident (Ident :: new ("core" , start))) ; tokens . append (TokenTree :: Punct (Punct :: new_spanned (':' , Spacing :: Joint , start ,))) ; tokens . append (TokenTree :: Punct (Punct :: new_spanned (':' , Spacing :: Alone , start ,))) ; tokens . append (TokenTree :: Ident (Ident :: new ("compile_error" , start))) ; tokens . append (TokenTree :: Punct (Punct :: new_spanned ('!' , Spacing :: Alone , start ,))) ; tokens . append (TokenTree :: Group ({ let mut group = Group :: new (Delimiter :: Brace , TokenStream :: from ({ let mut string = Literal :: string (& self . message) ; string . set_span (end) ; TokenTree :: Literal (string) }) ,) ; group . set_span (end) ; group })) ; } }
};
}
