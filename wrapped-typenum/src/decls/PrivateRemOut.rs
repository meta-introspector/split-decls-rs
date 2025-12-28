macro_rules! deps {
    () => {
        PrivateRem!();
    };
}

macro_rules! PrivateRemOut {
    () => {
        deps!();
        pub type PrivateRemOut < A , URem , Divisor > = < A as PrivateRem < URem , Divisor > > :: Output ;
    };
}

PrivateRemOut!()