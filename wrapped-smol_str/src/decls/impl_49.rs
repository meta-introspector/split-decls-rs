macro_rules! deps {
    () => {
        InlineSize!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl InlineSize { # [doc = " SAFETY: `value` must be less than or equal to [`INLINE_CAP`]"] # [inline (always)] const unsafe fn transmute_from_u8 (value : u8) -> Self { debug_assert ! (value <= InlineSize :: _V23 as u8) ; unsafe { mem :: transmute :: < u8 , Self > (value) } } }
    };
}

impl_49!();