// Generated macro for impl_397 (impl)
macro_rules! Depcrate_varzerovec_sliceimpl_397 {
() => {
// Module: crate::varzerovec::slice
// Provides: {"impl_397"}
// Dependencies: {}
impl < T : VarULE + ? Sized , F : VarZeroVecFormat > Index < usize > for VarZeroSlice < T , F > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { # [expect (clippy :: panic)] match self . get (index) { Some (x) => x , None => panic ! ("index out of bounds: the len is {} but the index is {index}" , self . len ()) , } } }
};
}
