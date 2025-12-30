// Generated macro for impl_35 (impl)
macro_rules! Depcrate_stable_vecimpl_35 {
() => {
// Module: crate::stable_vec
// Provides: {"impl_35"}
// Dependencies: {}
impl < T > Drop for StableVec < T > { fn drop (& mut self) { let _vec = unsafe { Vec :: from_raw_parts (self . addr as usize as * mut T , self . len as usize , self . cap as usize ,) } ; } }
};
}
