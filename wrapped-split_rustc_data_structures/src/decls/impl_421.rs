macro_rules! deps {
    () => {
        SnapshotMap!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < K , V > SnapshotMap < K , V > where K : Hash + Clone + Eq , { pub fn snapshot (& mut self) -> Snapshot { self . undo_log . start_snapshot () } pub fn commit (& mut self , snapshot : Snapshot) { self . undo_log . commit (snapshot) } pub fn rollback_to (& mut self , snapshot : Snapshot) { let map = & mut self . map ; self . undo_log . rollback_to (| | map , snapshot) } }
    };
}

impl_421!()