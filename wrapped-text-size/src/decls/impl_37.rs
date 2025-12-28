macro_rules! deps {
    () => {
        TextLen!();
        TextSize!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl TextLen for & '_ str { # [inline] fn text_len (self) -> TextSize { self . len () . try_into () . unwrap () } }
    };
}

impl_37!();