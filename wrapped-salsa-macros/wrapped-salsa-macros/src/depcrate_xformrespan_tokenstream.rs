// Generated macro for respan_tokenstream (function)
macro_rules! Depcrate_xformrespan_tokenstream {
() => {
// Module: crate::xform
// Provides: {"respan_tokenstream"}
// Dependencies: {}
fn respan_tokenstream (stream : proc_macro2 :: TokenStream , span : proc_macro2 :: Span ,) -> proc_macro2 :: TokenStream { stream . into_iter () . map (| token | respan_token (token , span)) . collect () }
};
}
