macro_rules! deps {
    () => {
        Z0!();
        Gcd!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Gcd < Z0 > for Z0 { type Output = Z0 ; }
    };
}

impl_118!();