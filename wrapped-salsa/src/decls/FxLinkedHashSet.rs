macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! FxLinkedHashSet {
    () => {
        deps!();
        pub (crate) type FxLinkedHashSet < K > = hashlink :: LinkedHashSet < K , FxHasher > ;
    };
}

FxLinkedHashSet!()