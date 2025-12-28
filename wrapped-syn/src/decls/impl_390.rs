macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl Receiver { pub fn lifetime (& self) -> Option < & Lifetime > { self . reference . as_ref () ? . 1 . as_ref () } }
    };
}

impl_390!();