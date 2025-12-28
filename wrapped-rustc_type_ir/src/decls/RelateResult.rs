macro_rules! deps {
    () => {
        TypeError!();
    };
}

macro_rules! RelateResult {
    () => {
        deps!();
        pub type RelateResult < I , T > = Result < T , TypeError < I > > ;
    };
}

RelateResult!();