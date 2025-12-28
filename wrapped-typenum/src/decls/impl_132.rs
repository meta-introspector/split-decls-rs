macro_rules! deps {
    () => {
        PInt!();
        Minimum!();
        NonZero!();
        Unsigned!();
        Min!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < Ul , Ur > Min < PInt < Ur > > for PInt < Ul > where Ul : Unsigned + NonZero + Min < Ur > , Ur : Unsigned + NonZero , Minimum < Ul , Ur > : Unsigned + NonZero , { type Output = PInt < Minimum < Ul , Ur > > ; # [inline] fn min (self , rhs : PInt < Ur >) -> Self :: Output { PInt { n : self . n . min (rhs . n) , } } }
    };
}

impl_132!();