macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < TokenText < '_ > > for String { fn from (token_text : TokenText < '_ >) -> Self { token_text . as_str () . into () } }
    };
}

impl_51!()