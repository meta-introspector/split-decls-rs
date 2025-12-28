macro_rules! deps {
    () => {
        UInt!();
        Cmp!();
        Bit!();
        PrivateMax!();
        Compare!();
        PrivateMaxOut!();
        Unsigned!();
        Max!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < U , B , Ur > Max < Ur > for UInt < U , B > where U : Unsigned , B : Bit , Ur : Unsigned , UInt < U , B > : Cmp < Ur > + PrivateMax < Ur , Compare < UInt < U , B > , Ur > > , { type Output = PrivateMaxOut < UInt < U , B > , Ur , Compare < UInt < U , B > , Ur > > ; # [inline] fn max (self , rhs : Ur) -> Self :: Output { self . private_max (rhs) } }
    };
}

impl_493!();