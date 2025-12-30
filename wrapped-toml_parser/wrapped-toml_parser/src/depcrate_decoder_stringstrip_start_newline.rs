// Generated macro for strip_start_newline (function)
macro_rules! Depcrate_decoder_stringstrip_start_newline {
() => {
// Module: crate::decoder::string
// Provides: {"strip_start_newline"}
// Dependencies: {}
fn strip_start_newline (s : & str) -> & str { s . strip_prefix ('\n') . or_else (| | s . strip_prefix ("\r\n")) . unwrap_or (s) }
};
}
