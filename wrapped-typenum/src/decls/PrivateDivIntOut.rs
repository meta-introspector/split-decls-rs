macro_rules! deps {
    () => {
        PrivateDivInt!();
    };
}

macro_rules! PrivateDivIntOut {
    () => {
        deps!();
        pub type PrivateDivIntOut < A , C , Divisor > = < A as PrivateDivInt < C , Divisor > > :: Output ;
    };
}

PrivateDivIntOut!()