macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < A : Array > From < ArrayVec < A > > for TinyVec < A > { # [inline (always)] fn from (arr : ArrayVec < A >) -> Self { TinyVec :: Inline (arr) } }
    };
}

impl_148!()