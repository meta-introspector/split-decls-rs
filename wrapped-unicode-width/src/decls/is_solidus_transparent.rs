macro_rules! is_solidus_transparent {
    () => {
        # [doc = " Whether this character is transparent wrt the effect of"] # [doc = " U+0338 COMBINING LONG SOLIDUS OVERLAY"] # [doc = " on its base character."] # [cfg (feature = "cjk")] fn is_solidus_transparent (c : char) -> bool { let cp : u32 = c . into () ; is_ligature_transparent (c) || SOLIDUS_TRANSPARENT . binary_search_by (| & (lo , hi) | { let lo = u32 :: from_le_bytes ([lo [0] , lo [1] , lo [2] , 0]) ; let hi = u32 :: from_le_bytes ([hi [0] , hi [1] , hi [2] , 0]) ; if cp < lo { Ordering :: Greater } else if cp > hi { Ordering :: Less } else { Ordering :: Equal } }) . is_ok () }
    };
}

is_solidus_transparent!()