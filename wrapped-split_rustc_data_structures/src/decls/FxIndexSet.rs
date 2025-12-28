macro_rules! FxIndexSet {
    () => {
        pub type FxIndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;
    };
}

FxIndexSet!()