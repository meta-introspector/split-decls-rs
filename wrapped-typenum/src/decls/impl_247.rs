macro_rules! deps {
    () => {
        Unsigned!();
        InvertedUInt!();
        PrivateInvert!();
        UInt!();
        Bit!();
        InvertedUnsigned!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < U : Unsigned , IU : InvertedUnsigned , B : Bit > PrivateInvert < U > for InvertedUInt < IU , B > where IU : PrivateInvert < UInt < U , B > > , { type Output = < IU as PrivateInvert < UInt < U , B > > > :: Output ; # [inline] fn private_invert (self , rhs : U) -> Self :: Output { self . msb . private_invert (UInt { msb : rhs , lsb : self . lsb , }) } }
    };
}

impl_247!()