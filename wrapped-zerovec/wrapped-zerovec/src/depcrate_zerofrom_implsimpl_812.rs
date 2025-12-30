// Generated macro for impl_812 (impl)
macro_rules! Depcrate_zerofrom_implsimpl_812 {
() => {
// Module: crate::zerofrom_impls
// Provides: {"impl_812"}
// Dependencies: {}
impl < 'zf , T > ZeroFrom < 'zf , ZeroVec < '_ , T > > for ZeroVec < 'zf , T > where T : 'static + AsULE , { # [inline] fn zero_from (other : & 'zf ZeroVec < '_ , T >) -> Self { ZeroVec :: new_borrowed (other . as_ule_slice ()) } }
};
}
