// Generated macro for impl_21 (impl)
macro_rules! Depcrate_mapimpl_21 {
() => {
// Module: crate::map
// Provides: {"impl_21"}
// Dependencies: {}
# [doc = " Access an element of this map. Panics if the given key is not present in the"] # [doc = " map."] impl < K , V , Q > ops :: Index < & Q > for Map < K , V > where K : Borrow < Q > + Ord , Q : Ord + Eq + Hash + ? Sized , { type Output = V ; fn index (& self , index : & Q) -> & V { self . map . index (index) } }
};
}
