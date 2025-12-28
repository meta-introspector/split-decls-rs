macro_rules! decompose {
    () => {
        # [inline] # [allow (unsafe_code)] fn decompose < D , F > (c : char , decompose_char : D , mut emit_char : F) where D : Fn (char) -> Option < & 'static [char] > , F : FnMut (char) , { if c <= '\x7f' { emit_char (c) ; return ; } if is_hangul_syllable (c) { unsafe { decompose_hangul (c , emit_char) ; } return ; } if let Some (decomposed) = decompose_char (c) { for & d in decomposed { emit_char (d) ; } return ; } emit_char (c) ; }
    };
}

decompose!()