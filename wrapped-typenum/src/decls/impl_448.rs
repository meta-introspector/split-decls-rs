macro_rules! deps {
    () => {
        Unsigned!();
        Gcd!();
        NonZero!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        # [doc = " gcd(x, 0) = x"] impl < X > Gcd < U0 > for X where X : Unsigned + NonZero , { type Output = X ; }
    };
}

impl_448!();