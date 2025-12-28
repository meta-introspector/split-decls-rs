macro_rules! deps {
    () => {
        Unsigned!();
        NInt!();
        Z0!();
        Min!();
        NonZero!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < U > Min < Z0 > for NInt < U > where U : Unsigned + NonZero , { type Output = NInt < U > ; # [inline] fn min (self , _ : Z0) -> Self :: Output { self } }
    };
}

impl_131!();