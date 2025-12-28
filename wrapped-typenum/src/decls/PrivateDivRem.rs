macro_rules! deps {
    () => {
        PrivateDiv!();
    };
}

macro_rules! PrivateDivRem {
    () => {
        deps!();
        pub type PrivateDivRem < N , D , Q , R , I > = < () as PrivateDiv < N , D , Q , R , I > > :: Remainder ;
    };
}

PrivateDivRem!();