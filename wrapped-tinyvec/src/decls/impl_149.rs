macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
        TinyVec!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < A : Array > From < A > for TinyVec < A > { # [inline] fn from (array : A) -> Self { TinyVec :: Inline (ArrayVec :: from (array)) } }
    };
}

impl_149!()