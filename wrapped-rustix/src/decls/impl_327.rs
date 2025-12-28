macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < T > :: core :: clone :: Clone for UnionField < T > { # [inline] fn clone (& self) -> Self { * self } }
    };
}

impl_327!();