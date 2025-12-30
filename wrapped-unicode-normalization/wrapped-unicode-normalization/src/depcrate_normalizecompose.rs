// Generated macro for compose (function)
macro_rules! Depcrate_normalizecompose {
() => {
// Module: crate::normalize
// Provides: {"compose"}
// Dependencies: {}
# [doc = " Compose two characters into a single character, if possible."] # [doc = " See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)"] # [doc = " for more information."] pub fn compose (a : char , b : char) -> Option < char > { compose_hangul (a , b) . or_else (| | composition_table (a , b)) }
};
}
