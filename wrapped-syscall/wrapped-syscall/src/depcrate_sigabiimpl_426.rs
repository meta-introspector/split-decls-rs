// Generated macro for impl_426 (impl)
macro_rules! Depcrate_sigabiimpl_426 {
() => {
// Module: crate::sigabi
// Provides: {"impl_426"}
// Dependencies: {}
impl SigatomicUsize { # [inline] pub fn load (& self , ordering : Ordering) -> usize { let value = self . 0 . load (Ordering :: Relaxed) ; if ordering != Ordering :: Relaxed { core :: sync :: atomic :: compiler_fence (ordering) ; } value } # [inline] pub fn store (& self , value : usize , ordering : Ordering) { if ordering != Ordering :: Relaxed { core :: sync :: atomic :: compiler_fence (ordering) ; } self . 0 . store (value , Ordering :: Relaxed) ; } }
};
}
