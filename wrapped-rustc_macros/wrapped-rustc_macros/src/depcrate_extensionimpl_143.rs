// Generated macro for impl_143 (impl)
macro_rules! Depcrate_extensionimpl_143 {
() => {
// Module: crate::extension
// Provides: {"impl_143"}
// Dependencies: {}
impl Parse for Impl { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; let _ : Token ! [impl] = input . parse () ? ; let generics = input . parse () ? ; let self_ty = input . parse () ? ; let wc = input . parse () ? ; let content ; let _brace_token = braced ! (content in input) ; let mut items = Vec :: new () ; while ! content . is_empty () { items . push (content . parse () ?) ; } Ok (Impl { attrs , generics , self_ty , items , wc }) } }
};
}
