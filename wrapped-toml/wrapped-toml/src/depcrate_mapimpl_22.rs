// Generated macro for impl_22 (impl)
macro_rules! Depcrate_mapimpl_22 {
() => {
// Module: crate::map
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " Mutably access an element of this map. Panics if the given key is not"] # [doc = " present in the map."] impl < K , V , Q > ops :: IndexMut < & Q > for Map < K , V > where K : Borrow < Q > + Ord , Q : Ord + Eq + Hash + ? Sized , { fn index_mut (& mut self , index : & Q) -> & mut V { self . map . get_mut (index) . expect ("no entry found for key") } }
};
}
