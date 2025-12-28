macro_rules! deps {
    () => {
        NonZero!();
        Maximum!();
        Unsigned!();
        PInt!();
        Max!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < Ul , Ur > Max < PInt < Ur > > for PInt < Ul > where Ul : Unsigned + NonZero + Max < Ur > , Ur : Unsigned + NonZero , Maximum < Ul , Ur > : Unsigned + NonZero , { type Output = PInt < Maximum < Ul , Ur > > ; # [inline] fn max (self , rhs : PInt < Ur >) -> Self :: Output { PInt { n : self . n . max (rhs . n) , } } }
    };
}

impl_141!()