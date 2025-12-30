// Generated macro for decompose_cjk_compat_variants (function)
macro_rules! Depcrate_normalizedecompose_cjk_compat_variants {
() => {
// Module: crate::normalize
// Provides: {"decompose_cjk_compat_variants"}
// Dependencies: {}
# [doc = " Compute standard-variation decomposition for character."] # [doc = ""] # [doc = " [Standardized Variation Sequences] are used instead of the standard canonical"] # [doc = " decompositions, notably for CJK codepoints with singleton canonical decompositions,"] # [doc = " to avoid losing information. See the"] # [doc = " [Unicode Variation Sequence FAQ](http://unicode.org/faq/vs.html) and the"] # [doc = " \"Other Enhancements\" section of the"] # [doc = " [Unicode 6.3 Release Summary](https://www.unicode.org/versions/Unicode6.3.0/#Summary)"] # [doc = " for more information."] # [inline] pub fn decompose_cjk_compat_variants < F > (c : char , mut emit_char : F) where F : FnMut (char) , { if c <= '\x7f' { emit_char (c) ; return ; } if let Some (decomposed) = cjk_compat_variants_fully_decomposed (c) { for & d in decomposed { emit_char (d) ; } return ; } emit_char (c) ; }
};
}
