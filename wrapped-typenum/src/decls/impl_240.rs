macro_rules! deps {
    () => {
        Bit!();
        Invert!();
        InvertedUInt!();
        InvertedUTerm!();
        PrivateInvert!();
        PrivateInvertOut!();
        Unsigned!();
        UInt!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < U : Unsigned , B : Bit > Invert for UInt < U , B > where U : PrivateInvert < InvertedUInt < InvertedUTerm , B > > , { type Output = PrivateInvertOut < U , InvertedUInt < InvertedUTerm , B > > ; # [inline] fn invert (self) -> Self :: Output { self . msb . private_invert (InvertedUInt { msb : InvertedUTerm , lsb : self . lsb , }) } }
    };
}

impl_240!();