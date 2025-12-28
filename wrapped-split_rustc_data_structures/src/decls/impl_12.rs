macro_rules! deps {
    () => {
        BaseNString!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl AsRef < str > for BaseNString { fn as_ref (& self) -> & str { self . buf [self . start ..] . as_str () } }
    };
}

impl_12!();