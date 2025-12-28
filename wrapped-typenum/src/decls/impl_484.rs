macro_rules! deps {
    () => {
        PrivateMin!();
        Bit!();
        Equal!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < U , B , Ur > PrivateMin < Ur , Equal > for UInt < U , B > where Ur : Unsigned , U : Unsigned , B : Bit , { type Output = UInt < U , B > ; # [inline] fn private_min (self , _ : Ur) -> Self :: Output { self } }
    };
}

impl_484!();