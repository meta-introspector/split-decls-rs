// Generated macro for impl_379 (impl)
macro_rules! Depcrate_varzerovec_ownedimpl_379 {
() => {
// Module: crate::varzerovec::owned
// Provides: {"impl_379"}
// Dependencies: {}
impl < 'a , T : ? Sized + VarULE , F : VarZeroVecFormat > From < & 'a VarZeroSlice < T , F > > for VarZeroVecOwned < T , F > { fn from (other : & 'a VarZeroSlice < T , F >) -> Self { Self :: from_slice (other) } }
};
}
