// Generated macro for impl_45 (impl)
macro_rules! Depcrate_helpers_metadataimpl_45 {
() => {
// Module: crate::helpers::metadata
// Provides: {"impl_45"}
// Dependencies: {}
impl Parse for InnerVariantMeta { fn parse (input : ParseStream) -> syn :: Result < Self > { let lookahead = input . lookahead1 () ; if lookahead . peek (kw :: default_with) { let kw = input . parse () ? ; let _ : Token ! [=] = input . parse () ? ; let value = input . parse () ? ; Ok (InnerVariantMeta :: DefaultWith { kw , value }) } else { Err (lookahead . error ()) } } }
};
}
