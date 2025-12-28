macro_rules! deps {
    () => {
        UTerm!();
        Bit!();
        Invert!();
        InvertedUInt!();
        PrivateInvert!();
        InvertedUnsigned!();
        UInt!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < IU : InvertedUnsigned , B : Bit > Invert for InvertedUInt < IU , B > where IU : PrivateInvert < UInt < UTerm , B > > , { type Output = < IU as PrivateInvert < UInt < UTerm , B > > > :: Output ; # [inline] fn invert (self) -> Self :: Output { self . msb . private_invert (UInt { msb : UTerm , lsb : self . lsb , }) } }
    };
}

impl_245!();