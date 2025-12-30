// Generated macro for impl_572 (impl)
macro_rules! Depcrate_sorted_mapimpl_572 {
() => {
// Module: crate::sorted_map
// Provides: {"impl_572"}
// Dependencies: {}
impl < K : Ord , V > IntoIterator for SortedMap < K , V > { type Item = (K , V) ; type IntoIter = std :: vec :: IntoIter < (K , V) > ; fn into_iter (self) -> Self :: IntoIter { self . data . into_iter () } }
};
}
