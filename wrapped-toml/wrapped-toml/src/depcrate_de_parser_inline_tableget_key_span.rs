// Generated macro for get_key_span (function)
macro_rules! Depcrate_de_parser_inline_tableget_key_span {
() => {
// Module: crate::de::parser::inline_table
// Provides: {"get_key_span"}
// Dependencies: {}
fn get_key_span (key : & Spanned < DeString < '_ > >) -> toml_parser :: Span { let key_span = key . span () ; toml_parser :: Span :: new_unchecked (key_span . start , key_span . end) }
};
}
