// Generated macro for is_utf8_encoding (function)
macro_rules! Depcrate_responseis_utf8_encoding {
() => {
// Module: crate::response
// Provides: {"is_utf8_encoding"}
// Dependencies: {}
# [doc = " Check if an encoding label refers to the UTF-8 encoding."] # [allow (dead_code)] fn is_utf8_encoding (encoding_label : & str) -> bool { encoding_label . eq_ignore_ascii_case ("utf-8") || encoding_label . eq_ignore_ascii_case ("utf8") || encoding_label . eq_ignore_ascii_case ("unicode-1-1-utf-8") }
};
}
