macro_rules! deps {
    () => {
        RustcPatCtxt!();
    };
}

macro_rules! WitnessPat {
    () => {
        deps!();
        pub type WitnessPat < 'p , 'tcx > = crate :: pat :: WitnessPat < RustcPatCtxt < 'p , 'tcx > > ;
    };
}

WitnessPat!();