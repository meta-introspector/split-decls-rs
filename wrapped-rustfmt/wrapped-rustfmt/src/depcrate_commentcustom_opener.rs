// Generated macro for custom_opener (function)
macro_rules! Depcrate_commentcustom_opener {
() => {
// Module: crate::comment
// Provides: {"custom_opener"}
// Dependencies: {}
fn custom_opener (s : & str) -> & str { s . lines () . next () . map_or ("" , | first_line | { first_line . find (' ') . map_or (first_line , | space_index | & first_line [0 ..= space_index]) }) }
};
}
