// Generated macro for impl_358 (impl)
macro_rules! Depcrate_query_cachesimpl_358 {
() => {
// Module: crate::query::caches
// Provides: {"impl_358"}
// Dependencies: {}
impl < V > QueryCache for DefIdCache < V > where V : Copy , { type Key = DefId ; type Value = V ; # [inline (always)] fn lookup (& self , key : & DefId) -> Option < (V , DepNodeIndex) > { if key . krate == LOCAL_CRATE { self . local . lookup (& key . index) } else { self . foreign . lookup (key) } } # [inline] fn complete (& self , key : DefId , value : V , index : DepNodeIndex) { if key . krate == LOCAL_CRATE { self . local . complete (key . index , value , index) } else { self . foreign . complete (key , value , index) } } fn iter (& self , f : & mut dyn FnMut (& Self :: Key , & Self :: Value , DepNodeIndex)) { self . local . iter (& mut | key , value , index | { f (& DefId { krate : LOCAL_CRATE , index : * key } , value , index) ; }) ; self . foreign . iter (f) ; } }
};
}
