macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < A : Array > Borrow < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn borrow (& self) -> & [A :: Item] { & * self } }
    };
}

impl_32!()