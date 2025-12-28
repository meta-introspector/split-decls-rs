macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! FxIndexSet {
    () => {
        deps!();
        pub (crate) type FxIndexSet < K > = indexmap :: IndexSet < K , FxHasher > ;
    };
}

FxIndexSet!()