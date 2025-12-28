macro_rules! deps {
    () => {
        Unhasher!();
    };
}

macro_rules! UnindexMap {
    () => {
        deps!();
        pub type UnindexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < Unhasher > > ;
    };
}

UnindexMap!()