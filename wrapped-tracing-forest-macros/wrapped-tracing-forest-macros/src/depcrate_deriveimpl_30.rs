// Generated macro for impl_30 (impl)
macro_rules! Depcrate_deriveimpl_30 {
() => {
// Module: crate::derive
// Provides: {"impl_30"}
// Dependencies: {}
impl ToTokens for TagRepr { fn to_tokens (& self , tokens : & mut TokenStream2) { let message = & self . message ; let icon = self . icon . as_ref () . map (| icon | quote ! { # icon }) . unwrap_or_else (| | { let level = self . level . quote_icon () ; quote ! { # level } }) ; quote ! { :: tracing_forest :: tag :: TagData { message : # message , icon : # icon } } . to_tokens (tokens) } }
};
}
