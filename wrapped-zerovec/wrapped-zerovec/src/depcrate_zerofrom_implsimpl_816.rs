// Generated macro for impl_816 (impl)
macro_rules! Depcrate_zerofrom_implsimpl_816 {
() => {
// Module: crate::zerofrom_impls
// Provides: {"impl_816"}
// Dependencies: {}
impl < 'zf , T , F : VarZeroVecFormat > ZeroFrom < 'zf , VarZeroVec < '_ , T , F > > for VarZeroVec < 'zf , T , F > where T : 'static + VarULE + ? Sized , { # [inline] fn zero_from (other : & 'zf VarZeroVec < '_ , T , F >) -> Self { other . as_slice () . into () } }
};
}
