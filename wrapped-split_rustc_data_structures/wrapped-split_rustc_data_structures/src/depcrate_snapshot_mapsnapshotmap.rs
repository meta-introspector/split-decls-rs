// Generated macro for SnapshotMap (struct)
macro_rules! Depcrate_snapshot_mapSnapshotMap {
() => {
// Module: crate::snapshot_map
// Provides: {"SnapshotMap"}
// Dependencies: {}
# [derive (Clone)] pub struct SnapshotMap < K , V , M = FxHashMap < K , V > , L = VecLog < UndoLog < K , V > > > { map : M , undo_log : L , _marker : PhantomData < (K , V) > , }
};
}
