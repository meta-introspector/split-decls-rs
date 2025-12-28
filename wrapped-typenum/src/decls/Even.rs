macro_rules! deps {
    () => {
        B0!();
        UInt!();
    };
}

macro_rules! Even {
    () => {
        deps!();
        # [doc = " The even number 2*N"] # [allow (unused)] type Even < N > = UInt < N , B0 > ;
    };
}

Even!()