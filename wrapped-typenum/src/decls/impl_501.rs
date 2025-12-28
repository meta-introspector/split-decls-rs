macro_rules! deps {
    () => {
        B1!();
        PrivateLogarithm2!();
        UInt!();
        Add1!();
        Bit!();
        Logarithm2!();
        Log2!();
        Unsigned!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < U , B > PrivateLogarithm2 for UInt < U , B > where U : Unsigned + Logarithm2 , B : Bit , Log2 < U > : Add < B1 > , { type Output = Add1 < Log2 < U > > ; }
    };
}

impl_501!();