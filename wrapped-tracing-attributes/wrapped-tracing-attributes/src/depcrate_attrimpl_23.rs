// Generated macro for impl_23 (impl)
macro_rules! Depcrate_attrimpl_23 {
() => {
// Module: crate::attr
// Provides: {"impl_23"}
// Dependencies: {}
impl Parse for LitStrOrIdent { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { input . parse :: < LitStr > () . map (LitStrOrIdent :: LitStr) . or_else (| _ | input . parse :: < Ident > () . map (LitStrOrIdent :: Ident)) } }
};
}
