// Generated macro for impl_253 (impl)
macro_rules! Depcrate_ref_countedimpl_253 {
() => {
// Module: crate::ref_counted
// Provides: {"impl_253"}
// Dependencies: {}
impl < T > Collectible for RefCounted < T > { # [inline] fn next_ptr (& self) -> Option < NonNull < dyn Collectible > > { self . next_or_refcnt . next_ptr () } # [inline] fn set_next_ptr (& self , next_ptr : Option < NonNull < dyn Collectible > >) { self . next_or_refcnt . set_next_ptr (next_ptr) ; } }
};
}
