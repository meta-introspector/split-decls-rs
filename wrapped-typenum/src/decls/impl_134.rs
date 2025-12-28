macro_rules! deps {
    () => {
        Min!();
        Unsigned!();
        NonZero!();
        PInt!();
        NInt!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < Ul , Ur > Min < NInt < Ur > > for PInt < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , { type Output = NInt < Ur > ; # [inline] fn min (self , rhs : NInt < Ur >) -> Self :: Output { rhs } }
    };
}

impl_134!()