macro_rules! deps {
    () => {
        Less!();
        Unsigned!();
        PrivateMax!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl < U , B , Ur > PrivateMax < Ur , Less > for UInt < U , B > where Ur : Unsigned , U : Unsigned , B : Bit , { type Output = Ur ; # [inline] fn private_max (self , rhs : Ur) -> Self :: Output { rhs } }
    };
}

impl_490!();