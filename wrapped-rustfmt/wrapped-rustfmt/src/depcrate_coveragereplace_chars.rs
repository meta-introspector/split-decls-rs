// Generated macro for replace_chars (function)
macro_rules! Depcrate_coveragereplace_chars {
() => {
// Module: crate::coverage
// Provides: {"replace_chars"}
// Dependencies: {}
fn replace_chars (s : & str) -> String { s . chars () . map (| ch | if ch . is_whitespace () { ch } else { 'X' }) . collect () }
};
}
