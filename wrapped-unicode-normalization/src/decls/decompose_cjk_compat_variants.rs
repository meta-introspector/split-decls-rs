macro_rules! decompose_cjk_compat_variants {
    () => {
        # [doc = " Compute standard-variation decomposition for character."] # [doc = ""] # [doc = " [Standardized Variation Sequences] are used instead of the standard canonical"] # [doc = " decompositions, notably for CJK codepoints with singleton canonical decompositions,"] # [doc = " to avoid losing information. See the [Unicode Variation Sequence FAQ] and the"] # [doc = " \"Other Enhancements\" section of the [Unicode 6.3 Release Summary] for more information."] # [doc = ""] # [doc = " [Standardized Variation Sequences]: https://www.unicode.org/glossary/#standardized_variation_sequence"] # [doc = " [Unicode Variation Sequence FAQ]: http://unicode.org/faq/vs.html"] # [doc = " [Unicode 6.3 Release Summary]: https://www.unicode.org/versions/Unicode6.3.0/#Summary"] # [inline] pub fn decompose_cjk_compat_variants < F > (c : char , mut emit_char : F) where F : FnMut (char) , { if c <= '\x7f' { emit_char (c) ; return ; } if let Some (decomposed) = cjk_compat_variants_fully_decomposed (c) { for & d in decomposed { emit_char (d) ; } return ; } emit_char (c) ; }
    };
}

decompose_cjk_compat_variants!();