macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
        Max!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl < U > Max < U > for UTerm where U : Unsigned , { type Output = U ; # [inline] fn max (self , rhs : U) -> Self :: Output { rhs } }
    };
}

impl_492!()