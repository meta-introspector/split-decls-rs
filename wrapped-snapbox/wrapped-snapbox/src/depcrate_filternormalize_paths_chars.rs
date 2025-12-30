// Generated macro for normalize_paths_chars (function)
macro_rules! Depcrate_filternormalize_paths_chars {
() => {
// Module: crate::filter
// Provides: {"normalize_paths_chars"}
// Dependencies: {}
fn normalize_paths_chars (data : impl Iterator < Item = char >) -> impl Iterator < Item = char > { data . map (| c | if c == '\\' { '/' } else { c }) }
};
}
