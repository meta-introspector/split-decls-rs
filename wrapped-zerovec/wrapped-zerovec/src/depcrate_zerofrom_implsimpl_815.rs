// Generated macro for impl_815 (impl)
macro_rules! Depcrate_zerofrom_implsimpl_815 {
() => {
// Module: crate::zerofrom_impls
// Provides: {"impl_815"}
// Dependencies: {}
impl < 'zf , T , F : VarZeroVecFormat > ZeroFrom < 'zf , VarZeroSlice < T , F > > for VarZeroVec < 'zf , T , F > where T : 'static + VarULE + ? Sized , { # [inline] fn zero_from (other : & 'zf VarZeroSlice < T , F >) -> Self { other . into () } }
};
}
