// Generated macro for impl_31 (impl)
macro_rules! Depcrate_deriveimpl_31 {
() => {
// Module: crate::derive
// Provides: {"impl_31"}
// Dependencies: {}
impl TagRepr { fn declare_macro (& self) -> Option < TokenStream2 > { self . tag_macro . as_ref () . map (| tag_macro | { let TagMacro { ident , variant_path , } = tag_macro ; let level = self . level . quote () ; quote ! { macro_rules ! # ident { ($ tokens : tt) => { :: tracing ::# level ! (__event_tag = :: tracing_forest :: Tag :: as_field (&$ crate :: tracing_forest_tag ::# variant_path) , $ tokens) } ; } } }) } }
};
}
