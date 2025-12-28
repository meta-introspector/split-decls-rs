macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < isize > for Index { # [inline (always)] fn from (index : isize) -> Self { Index :: new_isize (index) } }
    };
}

impl_29!()