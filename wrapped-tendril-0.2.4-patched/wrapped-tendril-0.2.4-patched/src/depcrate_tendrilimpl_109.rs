// Generated macro for impl_109 (impl)
macro_rules! Depcrate_tendrilimpl_109 {
() => {
// Module: crate::tendril
// Provides: {"impl_109"}
// Dependencies: {}
unsafe impl Atomicity for NonAtomic { # [inline] fn new () -> Self { NonAtomic (Cell :: new (1)) } # [inline] fn increment (& self) -> usize { let value = self . 0 . get () ; self . 0 . set (value . checked_add (1) . expect (OFLOW)) ; value } # [inline] fn decrement (& self) -> usize { let value = self . 0 . get () ; self . 0 . set (value - 1) ; value } # [inline] fn fence_acquire () { } }
};
}
