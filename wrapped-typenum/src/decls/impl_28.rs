macro_rules! deps {
    () => {
        B0!();
        B1!();
        Min!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Min < B0 > for B1 { type Output = B0 ; # [inline] fn min (self , rhs : B0) -> B0 { rhs } }
    };
}

impl_28!()