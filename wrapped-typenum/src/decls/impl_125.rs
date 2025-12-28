macro_rules! deps {
    () => {
        Unsigned!();
        PInt!();
        NonZero!();
        NInt!();
        Gcf!();
        Gcd!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < U1 , U2 > Gcd < NInt < U2 > > for PInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
    };
}

impl_125!()