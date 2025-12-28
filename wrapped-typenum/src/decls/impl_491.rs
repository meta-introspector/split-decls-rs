macro_rules! deps {
    () => {
        PrivateMax!();
        UInt!();
        Greater!();
        Bit!();
        Unsigned!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < U , B , Ur > PrivateMax < Ur , Greater > for UInt < U , B > where Ur : Unsigned , U : Unsigned , B : Bit , { type Output = UInt < U , B > ; # [inline] fn private_max (self , _ : Ur) -> Self :: Output { self } }
    };
}

impl_491!();