macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl ops :: Deref for TokenText < '_ > { type Target = str ; fn deref (& self) -> & str { self . as_str () } }
    };
}

impl_49!();