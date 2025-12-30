// Generated macro for HEAP_SLAB (static)
macro_rules! Depcrate_externrefHEAP_SLAB {
() => {
// Module: crate::externref
// Provides: {"HEAP_SLAB"}
// Dependencies: {}
# [cfg_attr (target_feature = "atomics" , thread_local)] static HEAP_SLAB : __rt :: ThreadLocalWrapper < RefCell < Slab > > = __rt :: ThreadLocalWrapper (RefCell :: new (Slab :: new ())) ;
};
}
