macro_rules! deps {
    () => {
        NInt!();
        Z0!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [doc = " `N * Z0 = Z0`"] impl < U : Unsigned + NonZero > Mul < Z0 > for NInt < U > { type Output = Z0 ; # [inline] fn mul (self , _ : Z0) -> Self :: Output { Z0 } }
    };
}

impl_80!()