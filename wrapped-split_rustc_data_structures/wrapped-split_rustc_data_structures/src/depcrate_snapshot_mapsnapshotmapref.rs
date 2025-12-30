// Generated macro for SnapshotMapRef (type)
macro_rules! Depcrate_snapshot_mapSnapshotMapRef {
() => {
// Module: crate::snapshot_map
// Provides: {"SnapshotMapRef"}
// Dependencies: {}
pub type SnapshotMapRef < 'a , K , V , L > = SnapshotMap < K , V , & 'a mut FxHashMap < K , V > , & 'a mut L > ;
};
}
