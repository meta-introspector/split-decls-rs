macro_rules! is_transparent_zero_width {
    () => {
        # [doc = " Whether this character is a zero-width character with"] # [doc = " `Joining_Type=Transparent`. Used by the Alef-Lamed ligatures."] # [doc = " See also [`is_ligature_transparent`], a near-subset of this (only ZWJ is excepted)"] # [doc = " which is transparent for non-Arabic ligatures."] fn is_transparent_zero_width (c : char) -> bool { if lookup_width (c) . 0 != 0 { false } else { let cp : u32 = c . into () ; NON_TRANSPARENT_ZERO_WIDTHS . binary_search_by (| & (lo , hi) | { let lo = u32 :: from_le_bytes ([lo [0] , lo [1] , lo [2] , 0]) ; let hi = u32 :: from_le_bytes ([hi [0] , hi [1] , hi [2] , 0]) ; if cp < lo { Ordering :: Greater } else if cp > hi { Ordering :: Less } else { Ordering :: Equal } }) . is_err () } }
    };
}

is_transparent_zero_width!()