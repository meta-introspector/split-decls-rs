macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl From < TokenText < '_ > > for SmolStr { fn from (token_text : TokenText < '_ >) -> Self { SmolStr :: new (token_text . as_str ()) } }
    };
}

impl_52!()