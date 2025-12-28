macro_rules! deps {
    () => {
        TextSize!();
        TextLen!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl TextSize { # [doc = " Creates a new instance of `TextSize` from a raw `u32`."] # [inline] pub const fn new (raw : u32) -> TextSize { TextSize { raw } } # [doc = " The text size of some primitive text-like object."] # [doc = ""] # [doc = " Accepts `char`, `&str`, and `&String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use text_size::*;"] # [doc = " let char_size = TextSize::of('🦀');"] # [doc = " assert_eq!(char_size, TextSize::from(4));"] # [doc = ""] # [doc = " let str_size = TextSize::of(\"rust-analyzer\");"] # [doc = " assert_eq!(str_size, TextSize::from(13));"] # [doc = " ```"] # [inline] pub fn of < T : TextLen > (text : T) -> TextSize { text . text_len () } }
    };
}

impl_21!();