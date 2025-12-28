macro_rules! deps {
    () => {
        Z0!();
        Unsigned!();
        NonZero!();
        Min!();
        NInt!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < U > Min < Z0 > for NInt < U > where U : Unsigned + NonZero , { type Output = NInt < U > ; # [inline] fn min (self , _ : Z0) -> Self :: Output { self } }
    };
}

impl_131!()