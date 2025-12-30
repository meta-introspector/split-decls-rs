// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < Z > Zeroize for Z where Z : DefaultIsZeroes , { fn zeroize (& mut self) { volatile_write (self , Z :: default ()) ; atomic_fence () ; } }
};
}
