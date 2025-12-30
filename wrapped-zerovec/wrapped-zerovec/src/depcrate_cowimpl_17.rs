// Generated macro for impl_17 (impl)
macro_rules! Depcrate_cowimpl_17 {
() => {
// Module: crate::cow
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a , V : ? Sized > Clone for VarZeroCow < 'a , V > { fn clone (& self) -> Self { let raw = self . raw . clone () ; unsafe { Self :: from_raw (raw) } } }
};
}
