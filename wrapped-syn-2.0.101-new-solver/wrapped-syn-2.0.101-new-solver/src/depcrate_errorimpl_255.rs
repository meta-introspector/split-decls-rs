// Generated macro for impl_255 (impl)
macro_rules! Depcrate_errorimpl_255 {
() => {
// Module: crate::error
// Provides: {"impl_255"}
// Dependencies: {}
impl ErrorMessage { fn to_compile_error (& self) -> TokenStream { let (start , end) = match self . span . get () { Some (range) => (range . start , range . end) , None => (Span :: call_site () , Span :: call_site ()) , } ; TokenStream :: from_iter ([TokenTree :: Punct ({ let mut punct = Punct :: new (':' , Spacing :: Joint) ; punct . set_span (start) ; punct }) , TokenTree :: Punct ({ let mut punct = Punct :: new (':' , Spacing :: Alone) ; punct . set_span (start) ; punct }) , TokenTree :: Ident (Ident :: new ("core" , start)) , TokenTree :: Punct ({ let mut punct = Punct :: new (':' , Spacing :: Joint) ; punct . set_span (start) ; punct }) , TokenTree :: Punct ({ let mut punct = Punct :: new (':' , Spacing :: Alone) ; punct . set_span (start) ; punct }) , TokenTree :: Ident (Ident :: new ("compile_error" , start)) , TokenTree :: Punct ({ let mut punct = Punct :: new ('!' , Spacing :: Alone) ; punct . set_span (start) ; punct }) , TokenTree :: Group ({ let mut group = Group :: new (Delimiter :: Brace , { TokenStream :: from_iter ([TokenTree :: Literal ({ let mut string = Literal :: string (& self . message) ; string . set_span (end) ; string })]) }) ; group . set_span (end) ; group }) ,]) } }
};
}
