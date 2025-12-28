macro_rules! deps {
    () => {
        PrivateCmp!();
    };
}

macro_rules! PrivateCmpOut {
    () => {
        deps!();
        pub type PrivateCmpOut < A , Rhs , SoFar > = < A as PrivateCmp < Rhs , SoFar > > :: Output ;
    };
}

PrivateCmpOut!()