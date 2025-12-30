// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < Z > Zeroize for Box < [Z] > where Z : Zeroize , { # [doc = " Unlike `Vec`, `Box<[Z]>` cannot reallocate, so we can be sure that we are not leaving"] # [doc = " values on the heap."] fn zeroize (& mut self) { self . iter_mut () . zeroize () ; } }
};
}
