// Generated macro for impl_18 (impl)
macro_rules! Depcrate_stable_vecimpl_18 {
() => {
// Module: crate::stable_vec
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > std :: ops :: Deref for StableVec < T > { type Target = [T] ; # [inline] fn deref (& self) -> & [T] { unsafe { core :: slice :: from_raw_parts (self . addr as usize as * mut T , self . len as usize) } } }
};
}
