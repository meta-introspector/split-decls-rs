macro_rules! deps {
    () => {
        Formatted!();
        Value!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl From < i64 > for Value { fn from (i : i64) -> Self { Self :: Integer (Formatted :: new (i)) } }
    };
}

impl_260!();