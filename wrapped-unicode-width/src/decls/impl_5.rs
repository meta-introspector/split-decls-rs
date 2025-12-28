macro_rules! deps {
    () => {
        UnicodeWidthStr!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl UnicodeWidthStr for str { # [inline] fn width (& self) -> usize { tables :: str_width (self) } # [cfg (feature = "cjk")] # [inline] fn width_cjk (& self) -> usize { tables :: str_width_cjk (self) } }
    };
}

impl_5!()