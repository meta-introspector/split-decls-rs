macro_rules! deps {
    () => {
        Less!();
        PrivateMin!();
        Unsigned!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl < U , B , Ur > PrivateMin < Ur , Less > for UInt < U , B > where Ur : Unsigned , U : Unsigned , B : Bit , { type Output = UInt < U , B > ; # [inline] fn private_min (self , _ : Ur) -> Self :: Output { self } }
    };
}

impl_485!();