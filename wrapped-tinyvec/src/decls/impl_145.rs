macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < A : Array > Borrow < [A :: Item] > for TinyVec < A > { # [inline (always)] fn borrow (& self) -> & [A :: Item] { & * self } }
    };
}

impl_145!()