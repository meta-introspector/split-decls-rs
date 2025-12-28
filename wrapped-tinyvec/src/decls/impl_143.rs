macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < A : Array > AsMut < [A :: Item] > for TinyVec < A > { # [inline (always)] fn as_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
    };
}

impl_143!()