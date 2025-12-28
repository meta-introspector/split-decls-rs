macro_rules! deps {
    () => {
        PrivateInvert!();
    };
}

macro_rules! PrivateInvertOut {
    () => {
        deps!();
        pub type PrivateInvertOut < A , Rhs > = < A as PrivateInvert < Rhs > > :: Output ;
    };
}

PrivateInvertOut!()