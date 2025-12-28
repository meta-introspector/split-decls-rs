macro_rules! IndexSet {
    () => {
        pub type IndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;
    };
}

IndexSet!();