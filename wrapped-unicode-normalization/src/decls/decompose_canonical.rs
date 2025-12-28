macro_rules! decompose_canonical {
    () => {
        # [doc = " Compute canonical Unicode decomposition for character."] # [doc = " See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)"] # [doc = " for more information."] # [inline] pub fn decompose_canonical < F > (c : char , emit_char : F) where F : FnMut (char) , { decompose (c , canonical_fully_decomposed , emit_char) }
    };
}

decompose_canonical!();