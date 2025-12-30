// Generated macro for impl_25 (impl)
macro_rules! Depcrate_mapimpl_25 {
() => {
// Module: crate::map
// Provides: {"impl_25"}
// Dependencies: {}
impl < K : Ord + Hash , V > Extend < (K , V) > for Map < K , V > { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = (K , V) > , { self . map . extend (iter) ; } }
};
}
