// Generated macro for impl_398 (impl)
macro_rules! Depcrate_varzerovec_sliceimpl_398 {
() => {
// Module: crate::varzerovec::slice
// Provides: {"impl_398"}
// Dependencies: {}
impl < T , F > PartialEq < VarZeroSlice < T , F > > for VarZeroSlice < T , F > where T : VarULE , T : ? Sized , T : PartialEq , F : VarZeroVecFormat , { # [inline] fn eq (& self , other : & VarZeroSlice < T , F >) -> bool { self . entire_slice . eq (& other . entire_slice) } }
};
}
