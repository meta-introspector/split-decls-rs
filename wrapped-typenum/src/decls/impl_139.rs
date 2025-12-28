macro_rules! deps {
    () => {
        PInt!();
        Unsigned!();
        Z0!();
        NonZero!();
        Max!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < U > Max < Z0 > for PInt < U > where U : Unsigned + NonZero , { type Output = PInt < U > ; # [inline] fn max (self , _ : Z0) -> Self :: Output { self } }
    };
}

impl_139!()