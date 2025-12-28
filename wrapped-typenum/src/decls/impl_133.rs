macro_rules! deps {
    () => {
        NInt!();
        PInt!();
        Unsigned!();
        NonZero!();
        Min!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < Ul , Ur > Min < PInt < Ur > > for NInt < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , { type Output = NInt < Ul > ; # [inline] fn min (self , _ : PInt < Ur >) -> Self :: Output { self } }
    };
}

impl_133!()