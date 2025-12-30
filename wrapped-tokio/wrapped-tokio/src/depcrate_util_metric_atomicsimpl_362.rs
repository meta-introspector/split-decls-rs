// Generated macro for impl_362 (impl)
macro_rules! Depcrate_util_metric_atomicsimpl_362 {
() => {
// Module: crate::util::metric_atomics
// Provides: {"impl_362"}
// Dependencies: {}
# [cfg_attr (not (all (tokio_unstable , feature = "rt")) , allow (dead_code))] impl MetricAtomicUsize { pub (crate) fn new (value : usize) -> Self { Self { value : AtomicUsize :: new (value) , } } pub (crate) fn load (& self , ordering : Ordering) -> usize { self . value . load (ordering) } pub (crate) fn store (& self , val : usize , ordering : Ordering) { self . value . store (val , ordering) } pub (crate) fn increment (& self) -> usize { self . value . fetch_add (1 , Ordering :: Relaxed) } pub (crate) fn decrement (& self) -> usize { self . value . fetch_sub (1 , Ordering :: Relaxed) } }
};
}
