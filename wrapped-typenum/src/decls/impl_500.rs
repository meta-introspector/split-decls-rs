macro_rules! deps {
    () => {
        B1!();
        PrivateLogarithm2!();
        UInt!();
        UTerm!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl PrivateLogarithm2 for UInt < UTerm , B1 > { type Output = U0 ; }
    };
}

impl_500!();