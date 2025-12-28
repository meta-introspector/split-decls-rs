macro_rules! deps {
    () => {
        Unsigned!();
        PInt!();
        NInt!();
        NonZero!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [doc = " `-NInt = PInt`"] impl < U : Unsigned + NonZero > Neg for NInt < U > { type Output = PInt < U > ; # [inline] fn neg (self) -> Self :: Output { PInt :: new () } }
    };
}

impl_58!();