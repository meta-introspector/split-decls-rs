macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl From < usize > for Index { # [inline (always)] fn from (index : usize) -> Self { Index :: new (index) } }
    };
}

impl_32!();