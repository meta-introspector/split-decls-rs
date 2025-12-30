// Generated macro for Buffer (trait)
macro_rules! Depcrate_direntBuffer {
() => {
// Module: crate::dirent
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " Abstraction between &mut [u8] and the kernel's UserSliceWo."] pub trait Buffer < 'a > : Sized + 'a { fn empty () -> Self ; fn length (& self) -> usize ; # [doc = " Split all of `self` into two disjoint contiguous subbuffers of lengths `index` and `length"] # [doc = " - index` respectively."] # [doc = ""] # [doc = " Returns None if and only if `index > length`."] fn split_at (self , index : usize) -> Option < [Self ; 2] > ; # [doc = " Copy from `src`, lengths must match exactly."] # [doc = ""] # [doc = " Allowed to overwrite subsequent buffer space, for performance reasons. Can be changed in"] # [doc = " the future if too restrictive."] fn copy_from_slice_exact (self , src : & [u8]) -> Result < () > ; # [doc = " Write zeroes to this part of the buffer."] # [doc = ""] # [doc = " Allowed to overwrite subsequent buffer space, for performance reasons. Can be changed in"] # [doc = " the future if too restrictive."] fn zero_out (self) -> Result < () > ; }
};
}
