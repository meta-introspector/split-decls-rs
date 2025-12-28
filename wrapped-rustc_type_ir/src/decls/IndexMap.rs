macro_rules! IndexMap {
    () => {
        pub type IndexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < FxHasher > > ;
    };
}

IndexMap!()