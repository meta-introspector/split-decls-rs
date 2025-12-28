macro_rules! deps {
    () => {
        PrivatePow!();
    };
}

macro_rules! PrivatePowOut {
    () => {
        deps!();
        pub type PrivatePowOut < A , Y , N > = < A as PrivatePow < Y , N > > :: Output ;
    };
}

PrivatePowOut!();