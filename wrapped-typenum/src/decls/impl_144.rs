macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        Min!();
        NInt!();
        Max!();
        Minimum!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < Ul , Ur > Max < NInt < Ur > > for NInt < Ul > where Ul : Unsigned + NonZero + Min < Ur > , Ur : Unsigned + NonZero , Minimum < Ul , Ur > : Unsigned + NonZero , { type Output = NInt < Minimum < Ul , Ur > > ; # [inline] fn max (self , rhs : NInt < Ur >) -> Self :: Output { NInt { n : self . n . min (rhs . n) , } } }
    };
}

impl_144!()