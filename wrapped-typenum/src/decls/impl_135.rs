macro_rules! deps {
    () => {
        Min!();
        Unsigned!();
        Maximum!();
        NonZero!();
        NInt!();
        Max!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < Ul , Ur > Min < NInt < Ur > > for NInt < Ul > where Ul : Unsigned + NonZero + Max < Ur > , Ur : Unsigned + NonZero , Maximum < Ul , Ur > : Unsigned + NonZero , { type Output = NInt < Maximum < Ul , Ur > > ; # [inline] fn min (self , rhs : NInt < Ur >) -> Self :: Output { NInt { n : self . n . max (rhs . n) , } } }
    };
}

impl_135!();