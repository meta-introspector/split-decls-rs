// Generated macro for impl_359 (impl)
macro_rules! Depcrate_query_cachesimpl_359 {
() => {
// Module: crate::query::caches
// Provides: {"impl_359"}
// Dependencies: {}
impl < K , V > QueryCache for VecCache < K , V , DepNodeIndex > where K : Idx + Eq + Hash + Copy + Debug , V : Copy , { type Key = K ; type Value = V ; # [inline (always)] fn lookup (& self , key : & K) -> Option < (V , DepNodeIndex) > { self . lookup (key) } # [inline] fn complete (& self , key : K , value : V , index : DepNodeIndex) { self . complete (key , value , index) } fn iter (& self , f : & mut dyn FnMut (& Self :: Key , & Self :: Value , DepNodeIndex)) { self . iter (f) } }
};
}
