macro_rules! deps {
    () => {
        RustcPatCtxt!();
    };
}

macro_rules! ConstructorSet {
    () => {
        deps!();
        pub type ConstructorSet < 'p , 'tcx > = crate :: constructor :: ConstructorSet < RustcPatCtxt < 'p , 'tcx > > ;
    };
}

ConstructorSet!()