macro_rules! deps {
    () => {
        PrivateDivIf!();
    };
}

macro_rules! PrivateDivIfQuot {
    () => {
        deps!();
        pub type PrivateDivIfQuot < N , D , Q , R , I , RcmpD > = < () as PrivateDivIf < N , D , Q , R , I , RcmpD > > :: Quotient ;
    };
}

PrivateDivIfQuot!()