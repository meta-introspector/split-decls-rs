macro_rules! deps {
    () => {
        Response!();
        Canonical!();
    };
}

macro_rules! CanonicalResponse {
    () => {
        deps!();
        pub type CanonicalResponse < I > = Canonical < I , Response < I > > ;
    };
}

CanonicalResponse!()