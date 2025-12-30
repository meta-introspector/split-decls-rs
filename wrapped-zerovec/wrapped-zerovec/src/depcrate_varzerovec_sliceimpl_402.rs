// Generated macro for impl_402 (impl)
macro_rules! Depcrate_varzerovec_sliceimpl_402 {
() => {
// Module: crate::varzerovec::slice
// Provides: {"impl_402"}
// Dependencies: {}
impl < T : VarULE + ? Sized , F : VarZeroVecFormat > fmt :: Debug for VarZeroSlice < T , F > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
