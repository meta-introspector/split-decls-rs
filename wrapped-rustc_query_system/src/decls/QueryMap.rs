macro_rules! deps {
    () => {
        QueryJobId!();
        QueryJobInfo!();
    };
}

macro_rules! QueryMap {
    () => {
        deps!();
        pub type QueryMap < I > = FxHashMap < QueryJobId , QueryJobInfo < I > > ;
    };
}

QueryMap!();