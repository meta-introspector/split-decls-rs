// Generated macro for make_error_message (function)
macro_rules! Depcrate_errormake_error_message {
() => {
// Module: crate::error
// Provides: {"make_error_message"}
// Dependencies: {}
unsafe fn make_error_message (msg : * mut libc :: c_char) -> Cow < 'static , str > { const FALLBACK : Cow < '_ , str > = Cow :: Borrowed ("<failed to fetch the error message>") ; :: opt_bytes (& () , msg) . and_then (| msg | { str :: from_utf8 (msg) . map (| msg | Cow :: Owned (msg . to_owned ())) . ok () }) . unwrap_or_else (| | FALLBACK) }
};
}
