// Generated macro for span_for_token_stream (function)
macro_rules! Depcrate_macrosspan_for_token_stream {
() => {
// Module: crate::macros
// Provides: {"span_for_token_stream"}
// Dependencies: {}
fn span_for_token_stream (token_stream : & TokenStream) -> Option < Span > { token_stream . iter () . next () . map (| tt | tt . span ()) }
};
}
