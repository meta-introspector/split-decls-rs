// Generated macro for impl_371 (impl)
macro_rules! Depcrate_varzerovec_ownedimpl_371 {
() => {
// Module: crate::varzerovec::owned
// Provides: {"impl_371"}
// Dependencies: {}
impl < T : ? Sized , F > Clone for VarZeroVecOwned < T , F > { fn clone (& self) -> Self { VarZeroVecOwned { marker1 : PhantomData , marker2 : PhantomData , entire_slice : self . entire_slice . clone () , } } }
};
}
