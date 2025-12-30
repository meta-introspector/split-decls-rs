// Generated macro for str_width (function)
macro_rules! Depcratestr_width {
() => {
// Module: crate
// Provides: {"str_width"}
// Dependencies: {}
pub fn str_width (s : & str) -> usize { s . chars () . map (char_width) . sum () }
};
}
