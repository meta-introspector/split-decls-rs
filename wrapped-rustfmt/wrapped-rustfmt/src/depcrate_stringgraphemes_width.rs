// Generated macro for graphemes_width (function)
macro_rules! Depcrate_stringgraphemes_width {
() => {
// Module: crate::string
// Provides: {"graphemes_width"}
// Dependencies: {}
fn graphemes_width (graphemes : & [& str]) -> usize { graphemes . iter () . map (| s | unicode_str_width (s)) . sum () }
};
}
