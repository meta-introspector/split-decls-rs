// Generated macro for impl_428 (impl)
macro_rules! Depcrate_sigabiimpl_428 {
() => {
// Module: crate::sigabi
// Provides: {"impl_428"}
// Dependencies: {}
impl NonatomicUsize { # [inline] pub const fn new (a : usize) -> Self { Self (AtomicUsize :: new (a)) } # [inline] pub fn get (& self) -> usize { self . 0 . load (Ordering :: Relaxed) } # [inline] pub fn set (& self , value : usize) { self . 0 . store (value , Ordering :: Relaxed) ; } }
};
}
