macro_rules! deps {
    () => {
        Decomposition!();
    };
}

macro_rules! classify_nonstarters {
    () => {
        deps!();
        # [inline] pub (crate) fn classify_nonstarters (c : char) -> Decomposition { if c <= '\x7f' { return Decomposition { leading_nonstarters : 0 , trailing_nonstarters : 0 , decomposition_len : 1 , } ; } if is_hangul_syllable (c) { return Decomposition { leading_nonstarters : 0 , trailing_nonstarters : 0 , decomposition_len : hangul_decomposition_length (c) , } ; } let decomp = compatibility_fully_decomposed (c) . or_else (| | canonical_fully_decomposed (c)) ; match decomp { Some (decomp) => Decomposition { leading_nonstarters : stream_safe_leading_nonstarters (c) , trailing_nonstarters : stream_safe_trailing_nonstarters (c) , decomposition_len : decomp . len () , } , None => { let is_nonstarter = canonical_combining_class (c) != 0 ; let nonstarter = if is_nonstarter { 1 } else { 0 } ; Decomposition { leading_nonstarters : nonstarter , trailing_nonstarters : nonstarter , decomposition_len : 1 , } } } }
    };
}

classify_nonstarters!();