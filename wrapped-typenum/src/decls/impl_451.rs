macro_rules! deps {
    () => {
        Even!();
        NonZero!();
        Gcf!();
        Gcd!();
        Odd!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        # [doc = " gcd(x, y) = gcd(x, y/2) if x odd and y even"] impl < Xp , Yp > Gcd < Even < Yp > > for Odd < Xp > where Odd < Xp > : Gcd < Yp > , Even < Yp > : NonZero , { type Output = Gcf < Odd < Xp > , Yp > ; }
    };
}

impl_451!()