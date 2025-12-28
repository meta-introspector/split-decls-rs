macro_rules! deps {
    () => {
        Value!();
        Formatted!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl From < bool > for Value { fn from (b : bool) -> Self { Self :: Boolean (Formatted :: new (b)) } }
    };
}

impl_262!();