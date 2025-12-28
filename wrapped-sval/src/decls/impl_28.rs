macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < i64 > for Index { # [inline (always)] fn from (index : i64) -> Self { Index :: new_i64 (index) } }
    };
}

impl_28!();