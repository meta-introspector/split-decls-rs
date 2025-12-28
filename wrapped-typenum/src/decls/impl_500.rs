macro_rules! deps {
    () => {
        UInt!();
        B1!();
        UTerm!();
        PrivateLogarithm2!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl PrivateLogarithm2 for UInt < UTerm , B1 > { type Output = U0 ; }
    };
}

impl_500!()