macro_rules! deps {
    () => {
        TextLen!();
        TextSize!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl TextLen for char { # [inline] fn text_len (self) -> TextSize { (self . len_utf8 () as u32) . into () } }
    };
}

impl_41!();