// Generated macro for impl_574 (impl)
macro_rules! Depcrate_sorted_mapimpl_574 {
() => {
// Module: crate::sorted_map
// Provides: {"impl_574"}
// Dependencies: {}
impl < 'a , K , Q , V > IndexMut < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { fn index_mut (& mut self , key : & Q) -> & mut Self :: Output { self . get_mut (key) . expect ("no entry found for key") } }
};
}
