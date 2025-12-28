macro_rules! deps {
    () => {
        Unsigned!();
        Max!();
        NInt!();
        NonZero!();
        PInt!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < Ul , Ur > Max < NInt < Ur > > for PInt < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , { type Output = PInt < Ul > ; # [inline] fn max (self , _ : NInt < Ur >) -> Self :: Output { self } }
    };
}

impl_143!();