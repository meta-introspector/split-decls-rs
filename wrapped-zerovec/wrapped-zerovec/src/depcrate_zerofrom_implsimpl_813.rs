// Generated macro for impl_813 (impl)
macro_rules! Depcrate_zerofrom_implsimpl_813 {
() => {
// Module: crate::zerofrom_impls
// Provides: {"impl_813"}
// Dependencies: {}
impl < 'zf , T > ZeroFrom < 'zf , ZeroSlice < T > > for ZeroVec < 'zf , T > where T : 'static + AsULE , { # [inline] fn zero_from (other : & 'zf ZeroSlice < T >) -> Self { ZeroVec :: new_borrowed (other . as_ule_slice ()) } }
};
}
