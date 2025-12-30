// Generated macro for impl_211 (impl)
macro_rules! Depcrate_collectibleimpl_211 {
() => {
// Module: crate::collectible
// Provides: {"impl_211"}
// Dependencies: {}
impl Link { # [inline] pub (super) const fn new_shared () -> Self { Link { data : (AtomicUsize :: new (1) , AtomicPtr :: new (ptr :: null_mut ())) , } } # [inline] pub (super) const fn new_unique () -> Self { Link { data : (AtomicUsize :: new (0) , AtomicPtr :: new (ptr :: null_mut ())) , } } # [inline] pub (super) const fn ref_cnt (& self) -> & AtomicUsize { & self . data . 0 } }
};
}
