// Generated macro for impl_229 (impl)
macro_rules! Depcrate_collectorimpl_229 {
() => {
// Module: crate::collector
// Provides: {"impl_229"}
// Dependencies: {}
impl Collectible for Collector { # [inline] fn next_ptr (& self) -> Option < NonNull < dyn Collectible > > { self . link . next_ptr () } # [inline] fn set_next_ptr (& self , next_ptr : Option < NonNull < dyn Collectible > >) { self . link . set_next_ptr (next_ptr) ; } }
};
}
