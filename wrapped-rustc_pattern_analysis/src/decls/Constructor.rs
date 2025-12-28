macro_rules! deps {
    () => {
        RustcPatCtxt!();
    };
}

macro_rules! Constructor {
    () => {
        deps!();
        pub type Constructor < 'p , 'tcx > = crate :: constructor :: Constructor < RustcPatCtxt < 'p , 'tcx > > ;
    };
}

Constructor!()