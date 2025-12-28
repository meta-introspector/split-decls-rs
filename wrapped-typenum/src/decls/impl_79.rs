macro_rules! deps {
    () => {
        PInt!();
        Unsigned!();
        NonZero!();
        Z0!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        # [doc = " `P * Z0 = Z0`"] impl < U : Unsigned + NonZero > Mul < Z0 > for PInt < U > { type Output = Z0 ; # [inline] fn mul (self , _ : Z0) -> Self :: Output { Z0 } }
    };
}

impl_79!()