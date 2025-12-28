macro_rules! deps {
    () => {
        PrivateDivIf!();
    };
}

macro_rules! PrivateDivIfRem {
    () => {
        deps!();
        pub type PrivateDivIfRem < N , D , Q , R , I , RcmpD > = < () as PrivateDivIf < N , D , Q , R , I , RcmpD > > :: Remainder ;
    };
}

PrivateDivIfRem!();