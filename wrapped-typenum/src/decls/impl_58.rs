macro_rules! deps {
    () => {
        NInt!();
        PInt!();
        NonZero!();
        Unsigned!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [doc = " `-NInt = PInt`"] impl < U : Unsigned + NonZero > Neg for NInt < U > { type Output = PInt < U > ; # [inline] fn neg (self) -> Self :: Output { PInt :: new () } }
    };
}

impl_58!()