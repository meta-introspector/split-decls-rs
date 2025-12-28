macro_rules! deps {
    () => {
        RustcPatCtxt!();
    };
}

macro_rules! DeconstructedPat {
    () => {
        deps!();
        pub type DeconstructedPat < 'p , 'tcx > = crate :: pat :: DeconstructedPat < RustcPatCtxt < 'p , 'tcx > > ;
    };
}

DeconstructedPat!();