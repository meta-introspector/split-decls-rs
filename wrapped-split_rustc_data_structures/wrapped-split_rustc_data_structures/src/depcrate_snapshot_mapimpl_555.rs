// Generated macro for impl_555 (impl)
macro_rules! Depcrate_snapshot_mapimpl_555 {
() => {
// Module: crate::snapshot_map
// Provides: {"impl_555"}
// Dependencies: {}
impl < 'k , K , V , M , L > ops :: Index < & 'k K > for SnapshotMap < K , V , M , L > where K : Hash + Clone + Eq , M : Borrow < FxHashMap < K , V > > , { type Output = V ; fn index (& self , key : & 'k K) -> & V { & self . map . borrow () [key] } }
};
}
