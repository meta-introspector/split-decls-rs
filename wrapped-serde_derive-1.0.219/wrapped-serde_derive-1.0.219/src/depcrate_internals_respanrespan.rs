// Generated macro for respan (function)
macro_rules! Depcrate_internals_respanrespan {
() => {
// Module: crate::internals::respan
// Provides: {"respan"}
// Dependencies: {}
pub (crate) fn respan (stream : TokenStream , span : Span) -> TokenStream { stream . into_iter () . map (| token | respan_token (token , span)) . collect () }
};
}
