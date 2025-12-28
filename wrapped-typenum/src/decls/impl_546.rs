macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        PInt!();
        ATerm!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < U > Mul < ATerm > for PInt < U > where U : Unsigned + NonZero , { type Output = ATerm ; # [inline] fn mul (self , _ : ATerm) -> Self :: Output { ATerm } }
    };
}

impl_546!()