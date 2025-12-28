macro_rules! deps {
    () => {
        NonZero!();
        Pow!();
        Unsigned!();
        PInt!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        # [doc = " P(Ul)^P(Ur) = P(Ul^Ur)"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Pow < PInt < Ur > > for PInt < Ul > where Ul : Pow < Ur > , < Ul as Pow < Ur > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Pow < Ur > > :: Output > ; # [inline] fn powi (self , _ : PInt < Ur >) -> Self :: Output { PInt :: new () } }
    };
}

impl_115!()