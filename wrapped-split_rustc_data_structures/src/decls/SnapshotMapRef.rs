macro_rules! deps {
    () => {
        SnapshotMap!();
    };
}

macro_rules! SnapshotMapRef {
    () => {
        deps!();
        pub type SnapshotMapRef < 'a , K , V , L > = SnapshotMap < K , V , & 'a mut FxHashMap < K , V > , & 'a mut L > ;
    };
}

SnapshotMapRef!()