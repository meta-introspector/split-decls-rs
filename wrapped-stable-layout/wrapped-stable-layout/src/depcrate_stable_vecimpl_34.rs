// Generated macro for impl_34 (impl)
macro_rules! Depcrate_stable_vecimpl_34 {
() => {
// Module: crate::stable_vec
// Provides: {"impl_34"}
// Dependencies: {}
impl < T > From < StableVec < T > > for Vec < T > { fn from (other : StableVec < T >) -> Self { let other = ManuallyDrop :: new (other) ; unsafe { Vec :: from_raw_parts (other . addr as usize as * mut T , other . len as usize , other . cap as usize ,) } } }
};
}
