macro_rules! deps {
    () => {
        Gcd!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        # [doc = " gcd(0, 0) = 0"] impl Gcd < U0 > for U0 { type Output = U0 ; }
    };
}

impl_447!()