// Generated macro for sanitize_name (function)
macro_rules! Depcratesanitize_name {
() => {
// Module: crate
// Provides: {"sanitize_name"}
// Dependencies: {}
# [doc = " Mangle a name into journald-compliant form"] fn sanitize_name (name : & str , buf : & mut Vec < u8 >) { buf . extend (name . bytes () . map (| c | if c == b'.' { b'_' } else { c }) . skip_while (| & c | c == b'_') . filter (| & c | c == b'_' || char :: from (c) . is_ascii_alphanumeric ()) . map (| c | char :: from (c) . to_ascii_uppercase () as u8) ,) ; }
};
}
