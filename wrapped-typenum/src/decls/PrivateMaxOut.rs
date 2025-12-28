macro_rules! deps {
    () => {
        PrivateMax!();
    };
}

macro_rules! PrivateMaxOut {
    () => {
        deps!();
        pub type PrivateMaxOut < A , B , CmpResult > = < A as PrivateMax < B , CmpResult > > :: Output ;
    };
}

PrivateMaxOut!();