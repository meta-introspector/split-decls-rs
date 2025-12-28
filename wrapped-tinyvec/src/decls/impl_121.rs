macro_rules! deps {
    () => {
        ArrayVec!();
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < A : Array > Default for TinyVec < A > { # [inline] fn default () -> Self { TinyVec :: Inline (ArrayVec :: default ()) } }
    };
}

impl_121!()