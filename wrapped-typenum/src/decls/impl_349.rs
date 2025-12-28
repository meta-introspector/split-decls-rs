macro_rules! deps {
    () => {
        Len!();
        Bit!();
        Length!();
        UInt!();
        Add1!();
        Unsigned!();
        B1!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        # [doc = " Length of a bit is 1"] impl < U : Unsigned , B : Bit > Len for UInt < U , B > where U : Len , Length < U > : Add < B1 > , Add1 < Length < U > > : Unsigned , { type Output = Add1 < Length < U > > ; # [inline] fn len (& self) -> Self :: Output { self . msb . len () + B1 } }
    };
}

impl_349!();