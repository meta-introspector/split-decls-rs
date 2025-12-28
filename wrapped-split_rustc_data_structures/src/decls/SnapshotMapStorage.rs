macro_rules! deps {
    () => {
        SnapshotMap!();
    };
}

macro_rules! SnapshotMapStorage {
    () => {
        deps!();
        pub type SnapshotMapStorage < K , V > = SnapshotMap < K , V , FxHashMap < K , V > , () > ;
    };
}

SnapshotMapStorage!();