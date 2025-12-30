// Generated macro for Iter (struct)
macro_rules! Depcrate_queueIter {
() => {
// Module: crate::queue
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a [`Queue`]."] # [doc = ""] # [doc = " [`Iter`] reads the oldest entry first."] pub struct Iter < 'g , T > { current : Ptr < 'g , LinkedEntry < T > > , guard : & 'g Guard , }
};
}
