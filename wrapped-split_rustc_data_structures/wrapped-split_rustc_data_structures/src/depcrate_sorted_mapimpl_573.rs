// Generated macro for impl_573 (impl)
macro_rules! Depcrate_sorted_mapimpl_573 {
() => {
// Module: crate::sorted_map
// Provides: {"impl_573"}
// Dependencies: {}
impl < 'a , K , Q , V > Index < & 'a Q > for SortedMap < K , V > where K : Ord + Borrow < Q > , Q : Ord + ? Sized , { type Output = V ; fn index (& self , key : & Q) -> & Self :: Output { self . get (key) . expect ("no entry found for key") } }
};
}
