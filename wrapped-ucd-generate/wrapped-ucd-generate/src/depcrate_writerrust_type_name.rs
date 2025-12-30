// Generated macro for rust_type_name (function)
macro_rules! Depcrate_writerrust_type_name {
() => {
// Module: crate::writer
// Provides: {"rust_type_name"}
// Dependencies: {}
# [doc = " Heuristically produce an appropriate Rust type name."] fn rust_type_name (s : & str) -> String { if s . chars () . all (| c | c . is_ascii_uppercase () || c . is_ascii_digit ()) { return s . to_string () ; } s . split (| c : char | c . is_whitespace () || c == '.' || c == '_' || c == '-') . map (| component | { let lower = component . to_ascii_lowercase () ; let mut chars = lower . chars () ; match chars . next () { None => String :: new () , Some (f) => { f . to_uppercase () . collect :: < String > () + chars . as_str () } } }) . collect () }
};
}
