// Generated macro for impl_214 (impl)
macro_rules! Depcrate_collectibleimpl_214 {
() => {
// Module: crate::collectible
// Provides: {"impl_214"}
// Dependencies: {}
impl < F : 'static + FnOnce () > Collectible for DeferredClosure < F > { # [inline] fn next_ptr (& self) -> Option < NonNull < dyn Collectible > > { self . link . next_ptr () } # [inline] fn set_next_ptr (& self , next_ptr : Option < NonNull < dyn Collectible > >) { self . link . set_next_ptr (next_ptr) ; } }
};
}
