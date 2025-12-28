macro_rules! deps {
    () => {
        Gcf!();
        B0!();
        Even!();
        Gcd!();
        UInt!();
        NonZero!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        # [doc = " gcd(x, y) = 2*gcd(x/2, y/2) if both x and y even"] impl < Xp , Yp > Gcd < Even < Yp > > for Even < Xp > where Xp : Gcd < Yp > , Even < Xp > : NonZero , Even < Yp > : NonZero , { type Output = UInt < Gcf < Xp , Yp > , B0 > ; }
    };
}

impl_450!();