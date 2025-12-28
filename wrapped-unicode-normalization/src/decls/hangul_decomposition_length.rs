macro_rules! hangul_decomposition_length {
    () => {
        # [inline] pub (crate) fn hangul_decomposition_length (s : char) -> usize { let si = s as u32 - S_BASE ; let ti = si % T_COUNT ; if ti > 0 { 3 } else { 2 } }
    };
}

hangul_decomposition_length!();