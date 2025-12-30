// Generated macro for impl_38 (impl)
macro_rules! Depcrate_attrimpl_38 {
() => {
// Module: crate::attr
// Provides: {"impl_38"}
// Dependencies: {}
impl ToTokens for FieldKind { fn to_tokens (& self , tokens : & mut TokenStream) { match self { FieldKind :: Debug => tokens . extend (quote ! { ? }) , FieldKind :: Display => tokens . extend (quote ! { % }) , _ => { } } } }
};
}
