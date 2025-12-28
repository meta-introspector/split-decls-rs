macro_rules! deps {
    () => {
        UnicodeWidthChar!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl UnicodeWidthChar for char { # [inline] fn width (self) -> Option < usize > { tables :: single_char_width (self) } # [cfg (feature = "cjk")] # [inline] fn width_cjk (self) -> Option < usize > { tables :: single_char_width_cjk (self) } }
    };
}

impl_54!()