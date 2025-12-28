macro_rules! deps {
    () => {
        Unsigned!();
        PInt!();
        NonZero!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [doc = " P(Ul) * P(Ur) = P(Ul * Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Mul < PInt < Ur > > for PInt < Ul > where Ul : Mul < Ur > , < Ul as Mul < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Mul < Ur > > :: Output > ; # [inline] fn mul (self , _ : PInt < Ur >) -> Self :: Output { PInt :: new () } }
    };
}

impl_81!();