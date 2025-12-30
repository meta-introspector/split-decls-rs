// Generated macro for is_new_line (function)
macro_rules! Depcrate_stringis_new_line {
() => {
// Module: crate::string
// Provides: {"is_new_line"}
// Dependencies: {}
fn is_new_line (grapheme : & str) -> bool { let bytes = grapheme . as_bytes () ; bytes . starts_with (b"\n") || bytes . starts_with (b"\r\n") }
};
}
