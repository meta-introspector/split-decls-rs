macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Eq for TokenText < '_ > { }
    };
}

impl_58!();