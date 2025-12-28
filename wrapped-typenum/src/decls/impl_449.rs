macro_rules! deps {
    () => {
        Unsigned!();
        Gcd!();
        NonZero!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        # [doc = " gcd(0, y) = y"] impl < Y > Gcd < Y > for U0 where Y : Unsigned + NonZero , { type Output = Y ; }
    };
}

impl_449!()