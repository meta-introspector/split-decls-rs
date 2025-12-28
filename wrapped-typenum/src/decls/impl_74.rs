macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        PInt!();
        NInt!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [doc = " `P(Ul) - N(Ur) = P(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < NInt < Ur > > for PInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn sub (self , _ : NInt < Ur >) -> Self :: Output { PInt :: new () } }
    };
}

impl_74!()