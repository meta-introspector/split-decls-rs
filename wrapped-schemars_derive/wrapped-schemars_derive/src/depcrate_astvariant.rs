// Generated macro for Variant (struct)
macro_rules! Depcrate_astVariant {
() => {
// Module: crate::ast
// Provides: {"Variant"}
// Dependencies: {}
pub struct Variant < 'a > { pub ident : syn :: Ident , pub serde_attrs : serde_derive_internals :: attr :: Variant , pub style : serde_ast :: Style , pub fields : Vec < Field < 'a > > , pub original : & 'a syn :: Variant , pub attrs : VariantAttrs , }
};
}
