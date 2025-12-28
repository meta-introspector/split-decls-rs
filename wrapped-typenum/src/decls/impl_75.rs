macro_rules! deps {
    () => {
        NonZero!();
        NInt!();
        PInt!();
        Unsigned!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [doc = " `N(Ul) - P(Ur) = N(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < PInt < Ur > > for NInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn sub (self , _ : PInt < Ur >) -> Self :: Output { NInt :: new () } }
    };
}

impl_75!();