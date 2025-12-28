macro_rules! deps {
    () => {
        PrivateAnd!();
    };
}

macro_rules! PrivateAndOut {
    () => {
        deps!();
        pub type PrivateAndOut < A , Rhs > = < A as PrivateAnd < Rhs > > :: Output ;
    };
}

PrivateAndOut!();