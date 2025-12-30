// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Zeroize for String { fn zeroize (& mut self) { unsafe { self . as_mut_vec () } . zeroize () ; } }
};
}
