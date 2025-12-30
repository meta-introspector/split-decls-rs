// Generated macro for GLOBAL_ROOT (static)
macro_rules! Depcrate_collectorGLOBAL_ROOT {
() => {
// Module: crate::collector
// Provides: {"GLOBAL_ROOT"}
// Dependencies: {}
# [doc = " The global and default [`CollectorRoot`]."] static GLOBAL_ROOT : CollectorRoot = CollectorRoot { epoch : AtomicU8 :: new (0) , chain_head : AtomicPtr :: new (ptr :: null_mut ()) , } ;
};
}
