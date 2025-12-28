macro_rules! deps {
    () => {
        PrivateMinOut!();
        Min!();
        PrivateMin!();
        Bit!();
        UInt!();
        Cmp!();
        Compare!();
        Unsigned!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < U , B , Ur > Min < Ur > for UInt < U , B > where U : Unsigned , B : Bit , Ur : Unsigned , UInt < U , B > : Cmp < Ur > + PrivateMin < Ur , Compare < UInt < U , B > , Ur > > , { type Output = PrivateMinOut < UInt < U , B > , Ur , Compare < UInt < U , B > , Ur > > ; # [inline] fn min (self , rhs : Ur) -> Self :: Output { self . private_min (rhs) } }
    };
}

impl_488!()