macro_rules! deps {
    () => {
        Unsigned!();
        Z0!();
        NInt!();
        NonZero!();
        Max!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < U > Max < Z0 > for NInt < U > where U : Unsigned + NonZero , { type Output = Z0 ; # [inline] fn max (self , rhs : Z0) -> Self :: Output { rhs } }
    };
}

impl_140!()