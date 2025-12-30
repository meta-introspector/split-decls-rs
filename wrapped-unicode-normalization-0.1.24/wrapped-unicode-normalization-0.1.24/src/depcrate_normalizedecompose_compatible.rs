// Generated macro for decompose_compatible (function)
macro_rules! Depcrate_normalizedecompose_compatible {
() => {
// Module: crate::normalize
// Provides: {"decompose_compatible"}
// Dependencies: {}
# [doc = " Compute canonical or compatible Unicode decomposition for character."] # [doc = " See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)"] # [doc = " for more information."] # [inline] pub fn decompose_compatible < F : FnMut (char) > (c : char , emit_char : F) { let decompose_char = | c | compatibility_fully_decomposed (c) . or_else (| | canonical_fully_decomposed (c)) ; decompose (c , decompose_char , emit_char) }
};
}
