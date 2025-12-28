macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl From < u64 > for Index { # [inline (always)] fn from (index : u64) -> Self { Index :: new_u64 (index) } }
    };
}

impl_31!();