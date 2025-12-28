macro_rules! deps {
    () => {
        Z0!();
        PInt!();
        NonZero!();
        Unsigned!();
        Min!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < U > Min < Z0 > for PInt < U > where U : Unsigned + NonZero , { type Output = Z0 ; # [inline] fn min (self , rhs : Z0) -> Self :: Output { rhs } }
    };
}

impl_130!()