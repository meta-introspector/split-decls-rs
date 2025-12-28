macro_rules! deps {
    () => {
        NonZero!();
        NInt!();
        Unsigned!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        # [doc = " `N(Ul) + N(Ur) = N(Ul + Ur)`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Add < NInt < Ur > > for NInt < Ul > where Ul : Add < Ur > , < Ul as Add < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Add < Ur > > :: Output > ; # [inline] fn add (self , _ : NInt < Ur >) -> Self :: Output { NInt :: new () } }
    };
}

impl_63!();