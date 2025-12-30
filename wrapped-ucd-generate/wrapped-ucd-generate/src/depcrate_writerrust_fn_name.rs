// Generated macro for rust_fn_name (function)
macro_rules! Depcrate_writerrust_fn_name {
() => {
// Module: crate::writer
// Provides: {"rust_fn_name"}
// Dependencies: {}
fn rust_fn_name (s : & str) -> String { s . to_ascii_lowercase () . chars () . map (| c | { if c . is_whitespace () || c == '.' || c == '-' { '_' } else { c } } ,) . collect () }
};
}
