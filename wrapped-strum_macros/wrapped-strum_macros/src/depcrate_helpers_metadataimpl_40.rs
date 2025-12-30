// Generated macro for impl_40 (impl)
macro_rules! Depcrate_helpers_metadataimpl_40 {
() => {
// Module: crate::helpers::metadata
// Provides: {"impl_40"}
// Dependencies: {}
impl Parse for Prop { fn parse (input : ParseStream) -> syn :: Result < Self > { use syn :: ext :: IdentExt ; let k = Ident :: parse_any (input) ? ; let _ : Token ! [=] = input . parse () ? ; let v = input . parse () ? ; Ok (Prop (k , v)) } }
};
}
