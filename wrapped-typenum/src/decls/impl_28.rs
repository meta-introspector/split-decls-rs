macro_rules! deps {
    () => {
        Min!();
        B1!();
        B0!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Min < B0 > for B1 { type Output = B0 ; # [inline] fn min (self , rhs : B0) -> B0 { rhs } }
    };
}

impl_28!();