macro_rules! deps {
    () => {
        PrivateMin!();
    };
}

macro_rules! PrivateMinOut {
    () => {
        deps!();
        pub type PrivateMinOut < A , B , CmpResult > = < A as PrivateMin < B , CmpResult > > :: Output ;
    };
}

PrivateMinOut!()