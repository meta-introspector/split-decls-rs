macro_rules! deps {
    () => {
        PrivateIntegerAdd!();
    };
}

macro_rules! PrivateIntegerAddOut {
    () => {
        deps!();
        pub type PrivateIntegerAddOut < P , C , N > = < P as PrivateIntegerAdd < C , N > > :: Output ;
    };
}

PrivateIntegerAddOut!();