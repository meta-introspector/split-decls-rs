macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < A : Array > Default for ArrayVec < A > { # [inline] fn default () -> Self { Self { len : 0 , data : A :: default () } } }
    };
}

impl_9!();