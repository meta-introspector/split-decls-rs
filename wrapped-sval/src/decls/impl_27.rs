macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < i32 > for Index { # [inline (always)] fn from (index : i32) -> Self { Index :: new_i32 (index) } }
    };
}

impl_27!()