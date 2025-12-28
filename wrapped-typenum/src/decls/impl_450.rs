macro_rules! deps {
    () => {
        NonZero!();
        B0!();
        Gcf!();
        Even!();
        Gcd!();
        UInt!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        # [doc = " gcd(x, y) = 2*gcd(x/2, y/2) if both x and y even"] impl < Xp , Yp > Gcd < Even < Yp > > for Even < Xp > where Xp : Gcd < Yp > , Even < Xp > : NonZero , Even < Yp > : NonZero , { type Output = UInt < Gcf < Xp , Yp > , B0 > ; }
    };
}

impl_450!()