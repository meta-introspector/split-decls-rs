// Generated macro for impl_554 (impl)
macro_rules! Depcrate_snapshot_mapimpl_554 {
() => {
// Module: crate::snapshot_map
// Provides: {"impl_554"}
// Dependencies: {}
impl < K , V > SnapshotMap < K , V > where K : Hash + Clone + Eq , { pub fn snapshot (& mut self) -> Snapshot { self . undo_log . start_snapshot () } pub fn commit (& mut self , snapshot : Snapshot) { self . undo_log . commit (snapshot) } pub fn rollback_to (& mut self , snapshot : Snapshot) { let map = & mut self . map ; self . undo_log . rollback_to (| | map , snapshot) } }
};
}
