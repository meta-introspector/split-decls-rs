macro_rules! deps {
    () => {
        Unhasher!();
    };
}

macro_rules! UnhashSet {
    () => {
        deps!();
        pub type UnhashSet < V > = HashSet < V , BuildHasherDefault < Unhasher > > ;
    };
}

UnhashSet!();