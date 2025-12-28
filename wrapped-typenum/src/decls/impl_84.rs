macro_rules! deps {
    () => {
        PInt!();
        NInt!();
        NonZero!();
        Unsigned!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [doc = " N(Ul) * P(Ur) = N(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < PInt < Ur > > for NInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : PInt < Ur >) -> Self :: Output { NInt :: new () } }
    };
}

impl_84!();