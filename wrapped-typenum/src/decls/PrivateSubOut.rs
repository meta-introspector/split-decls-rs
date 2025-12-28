macro_rules! deps {
    () => {
        PrivateSub!();
    };
}

macro_rules! PrivateSubOut {
    () => {
        deps!();
        pub type PrivateSubOut < A , Rhs > = < A as PrivateSub < Rhs > > :: Output ;
    };
}

PrivateSubOut!()