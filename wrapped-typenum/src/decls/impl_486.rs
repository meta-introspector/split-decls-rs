macro_rules! deps {
    () => {
        PrivateMin!();
        Bit!();
        Greater!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl < U , B , Ur > PrivateMin < Ur , Greater > for UInt < U , B > where Ur : Unsigned , U : Unsigned , B : Bit , { type Output = Ur ; # [inline] fn private_min (self , rhs : Ur) -> Self :: Output { rhs } }
    };
}

impl_486!()