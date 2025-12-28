macro_rules! deps {
    () => {
        Formatted!();
        Value!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl From < f64 > for Value { fn from (f : f64) -> Self { Self :: Float (Formatted :: new (f)) } }
    };
}

impl_261!()