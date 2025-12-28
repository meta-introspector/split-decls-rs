macro_rules! deps {
    () => {
        Unhasher!();
    };
}

macro_rules! UnhashMap {
    () => {
        deps!();
        pub type UnhashMap < K , V > = HashMap < K , V , BuildHasherDefault < Unhasher > > ;
    };
}

UnhashMap!();