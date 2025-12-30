// Generated macro for impl_557 (impl)
macro_rules! Depcrate_snapshot_mapimpl_557 {
() => {
// Module: crate::snapshot_map
// Provides: {"impl_557"}
// Dependencies: {}
impl < K , V > Rollback < UndoLog < K , V > > for FxHashMap < K , V > where K : Eq + Hash , { fn reverse (& mut self , undo : UndoLog < K , V >) { match undo { UndoLog :: Inserted (key) => { self . remove (& key) ; } UndoLog :: Overwrite (key , old_value) => { self . insert (key , old_value) ; } UndoLog :: Purged => { } } } }
};
}
