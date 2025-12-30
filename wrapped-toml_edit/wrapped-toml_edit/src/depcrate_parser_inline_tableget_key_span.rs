// Generated macro for get_key_span (function)
macro_rules! Depcrate_parser_inline_tableget_key_span {
() => {
// Module: crate::parser::inline_table
// Provides: {"get_key_span"}
// Dependencies: {}
fn get_key_span (key : & Key) -> Option < toml_parser :: Span > { key . as_repr () . and_then (| r | r . span ()) . map (| s | toml_parser :: Span :: new_unchecked (s . start , s . end)) }
};
}
