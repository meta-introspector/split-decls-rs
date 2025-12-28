macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl AsRef < str > for TokenText < '_ > { fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_50!()