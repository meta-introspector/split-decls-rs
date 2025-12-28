macro_rules! deps {
    () => {
        Gcf!();
        PInt!();
        Gcd!();
        NInt!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < U1 , U2 > Gcd < NInt < U2 > > for NInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
    };
}

impl_126!();