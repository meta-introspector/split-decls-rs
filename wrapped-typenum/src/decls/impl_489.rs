macro_rules! deps {
    () => {
        Unsigned!();
        PrivateMax!();
        Equal!();
        UInt!();
        Bit!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < U , B , Ur > PrivateMax < Ur , Equal > for UInt < U , B > where Ur : Unsigned , U : Unsigned , B : Bit , { type Output = UInt < U , B > ; # [inline] fn private_max (self , _ : Ur) -> Self :: Output { self } }
    };
}

impl_489!()