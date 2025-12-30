// Generated macro for impl_111 (impl)
macro_rules! Depcrate_tendrilimpl_111 {
() => {
// Module: crate::tendril
// Provides: {"impl_111"}
// Dependencies: {}
unsafe impl Atomicity for Atomic { # [inline] fn new () -> Self { Atomic (AtomicUsize :: new (1)) } # [inline] fn increment (& self) -> usize { self . 0 . fetch_add (1 , AtomicOrdering :: Relaxed) } # [inline] fn decrement (& self) -> usize { self . 0 . fetch_sub (1 , AtomicOrdering :: Release) } # [inline] fn fence_acquire () { atomic :: fence (AtomicOrdering :: Acquire) ; } }
};
}
