macro_rules! deps {
    () => {
        PrivateDiv!();
    };
}

macro_rules! PrivateDivQuot {
    () => {
        deps!();
        pub type PrivateDivQuot < N , D , Q , R , I > = < () as PrivateDiv < N , D , Q , R , I > > :: Quotient ;
    };
}

PrivateDivQuot!()