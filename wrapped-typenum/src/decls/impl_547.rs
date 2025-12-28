macro_rules! deps {
    () => {
        NInt!();
        Unsigned!();
        NonZero!();
        ATerm!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        impl < U > Mul < ATerm > for NInt < U > where U : Unsigned + NonZero , { type Output = ATerm ; # [inline] fn mul (self , _ : ATerm) -> Self :: Output { ATerm } }
    };
}

impl_547!();