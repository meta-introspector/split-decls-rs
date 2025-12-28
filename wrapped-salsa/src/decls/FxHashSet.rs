macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! FxHashSet {
    () => {
        deps!();
        pub (crate) type FxHashSet < K > = std :: collections :: HashSet < K , FxHasher > ;
    };
}

FxHashSet!();