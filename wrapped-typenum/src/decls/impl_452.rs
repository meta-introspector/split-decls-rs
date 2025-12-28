macro_rules! deps {
    () => {
        Gcd!();
        Odd!();
        Even!();
        NonZero!();
        Gcf!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        # [doc = " gcd(x, y) = gcd(x/2, y) if x even and y odd"] impl < Xp , Yp > Gcd < Odd < Yp > > for Even < Xp > where Xp : Gcd < Odd < Yp > > , Even < Xp > : NonZero , { type Output = Gcf < Xp , Odd < Yp > > ; }
    };
}

impl_452!();