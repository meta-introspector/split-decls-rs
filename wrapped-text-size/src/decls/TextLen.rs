macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! TextLen {
    () => {
        deps!();
        # [doc = " Primitives with a textual length that can be passed to [`TextSize::of`]."] pub trait TextLen : Copy + Sealed { # [doc = " The textual length of this primitive."] fn text_len (self) -> TextSize ; }
    };
}

TextLen!()