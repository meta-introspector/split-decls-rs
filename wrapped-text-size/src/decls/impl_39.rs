macro_rules! deps {
    () => {
        TextLen!();
        TextSize!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl TextLen for & '_ String { # [inline] fn text_len (self) -> TextSize { self . as_str () . text_len () } }
    };
}

impl_39!()