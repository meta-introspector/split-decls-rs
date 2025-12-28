macro_rules! FxIndexMap {
    () => {
        pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < FxHasher > > ;
    };
}

FxIndexMap!();