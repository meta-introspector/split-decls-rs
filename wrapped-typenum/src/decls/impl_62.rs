macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        PInt!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        # [doc = " `P(Ul) + P(Ur) = P(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Add < PInt < Ur > > for PInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn add (self , _ : PInt < Ur >) -> Self :: Output { PInt :: new () } }
    };
}

impl_62!();