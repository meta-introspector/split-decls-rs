macro_rules! deps {
    () => {
        PrivateXor!();
    };
}

macro_rules! PrivateXorOut {
    () => {
        deps!();
        pub type PrivateXorOut < A , Rhs > = < A as PrivateXor < Rhs > > :: Output ;
    };
}

PrivateXorOut!();