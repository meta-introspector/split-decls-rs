macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < u32 > for Index { # [inline (always)] fn from (index : u32) -> Self { Index :: new_u32 (index) } }
    };
}

impl_30!();