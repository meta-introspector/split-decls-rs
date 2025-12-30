// Generated macro for impl_19 (impl)
macro_rules! Depcrate_stable_vecimpl_19 {
() => {
// Module: crate::stable_vec
// Provides: {"impl_19"}
// Dependencies: {}
impl < T > std :: ops :: DerefMut for StableVec < T > { # [inline] fn deref_mut (& mut self) -> & mut [T] { unsafe { core :: slice :: from_raw_parts_mut (self . addr as usize as * mut T , self . len as usize) } } }
};
}
