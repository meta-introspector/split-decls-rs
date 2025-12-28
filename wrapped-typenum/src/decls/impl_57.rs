macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        NInt!();
        PInt!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [doc = " `-PInt = NInt`"] impl < U : Unsigned + NonZero > Neg for PInt < U > { type Output = NInt < U > ; # [inline] fn neg (self) -> Self :: Output { NInt :: new () } }
    };
}

impl_57!()