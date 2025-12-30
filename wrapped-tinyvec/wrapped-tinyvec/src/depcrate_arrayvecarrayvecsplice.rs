// Generated macro for ArrayVecSplice (struct)
macro_rules! Depcrate_arrayvecArrayVecSplice {
() => {
// Module: crate::arrayvec
// Provides: {"ArrayVecSplice"}
// Dependencies: {}
# [doc = " Splicing iterator for `ArrayVec`"] # [doc = " See [`ArrayVec::splice`](ArrayVec::<A>::splice)"] pub struct ArrayVecSplice < 'p , A : Array , I : Iterator < Item = A :: Item > > { parent : & 'p mut ArrayVec < A > , removal_start : usize , removal_end : usize , replacement : I , }
};
}
