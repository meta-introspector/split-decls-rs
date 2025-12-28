macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < A : Array > AsMut < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn as_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
    };
}

impl_30!()