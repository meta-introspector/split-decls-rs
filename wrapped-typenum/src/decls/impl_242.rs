macro_rules! deps {
    () => {
        UInt!();
        Bit!();
        Unsigned!();
        PrivateInvertOut!();
        PrivateInvert!();
        InvertedUnsigned!();
        InvertedUInt!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < IU : InvertedUnsigned , U : Unsigned , B : Bit > PrivateInvert < IU > for UInt < U , B > where U : PrivateInvert < InvertedUInt < IU , B > > , { type Output = PrivateInvertOut < U , InvertedUInt < IU , B > > ; # [inline] fn private_invert (self , rhs : IU) -> Self :: Output { self . msb . private_invert (InvertedUInt { msb : rhs , lsb : self . lsb , }) } }
    };
}

impl_242!()