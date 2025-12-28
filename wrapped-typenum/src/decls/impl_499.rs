macro_rules! deps {
    () => {
        PrivateLogarithm2!();
        Logarithm2!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl < N > Logarithm2 for N where N : PrivateLogarithm2 , { type Output = < Self as PrivateLogarithm2 > :: Output ; }
    };
}

impl_499!()