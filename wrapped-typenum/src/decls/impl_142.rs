macro_rules! deps {
    () => {
        PInt!();
        NInt!();
        NonZero!();
        Max!();
        Unsigned!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < Ul , Ur > Max < PInt < Ur > > for NInt < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , { type Output = PInt < Ur > ; # [inline] fn max (self , rhs : PInt < Ur >) -> Self :: Output { rhs } }
    };
}

impl_142!()