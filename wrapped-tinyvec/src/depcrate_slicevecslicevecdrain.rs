// Generated macro for SliceVecDrain (struct)
macro_rules! Depcrate_slicevecSliceVecDrain {
() => {
// Module: crate::slicevec
// Provides: {"SliceVecDrain"}
// Dependencies: {}
# [doc = " Draining iterator for [`SliceVec`]"] # [doc = ""] # [doc = " See [`SliceVec::drain`](SliceVec::drain)"] pub struct SliceVecDrain < 'p , 's , T : Default > { parent : & 'p mut SliceVec < 's , T > , target_start : usize , target_index : usize , target_end : usize , }
};
}
