macro_rules! deps {
    () => {
        InlineSize!();
        SmolStr!();
        Repr!();
    };
}

macro_rules! replacen_1_ascii {
    () => {
        deps!();
        # [doc = " SAFETY: `map` fn must only replace ascii with ascii or return unchanged bytes."] # [inline] unsafe fn replacen_1_ascii (src : & str , mut map : impl FnMut (& u8) -> u8) -> SmolStr { if src . len () <= INLINE_CAP { let mut buf = [0u8 ; INLINE_CAP] ; for (idx , b) in src . as_bytes () . iter () . enumerate () { buf [idx] = map (b) ; } SmolStr (Repr :: Inline { len : unsafe { InlineSize :: transmute_from_u8 (src . len () as u8) } , buf , }) } else { let out = src . as_bytes () . iter () . map (map) . collect () ; unsafe { String :: from_utf8_unchecked (out) . into () } } }
    };
}

replacen_1_ascii!()