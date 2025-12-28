macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < A : Array > AsRef < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn as_ref (& self) -> & [A :: Item] { & * self } }
    };
}

impl_31!()