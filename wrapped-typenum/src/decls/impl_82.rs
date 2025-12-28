macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        PInt!();
        NInt!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [doc = " N(Ul) * N(Ur) = P(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < NInt < Ur > > for NInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : NInt < Ur >) -> Self :: Output { PInt :: new () } }
    };
}

impl_82!();