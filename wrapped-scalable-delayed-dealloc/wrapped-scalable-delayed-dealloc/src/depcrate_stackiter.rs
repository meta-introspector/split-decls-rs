// Generated macro for Iter (struct)
macro_rules! Depcrate_stackIter {
() => {
// Module: crate::stack
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a [`Stack`]."] # [doc = ""] # [doc = " [`Iter`] reads the newest entry first."] pub struct Iter < 'g , T > { current : Ptr < 'g , LinkedEntry < T > > , guard : & 'g Guard , }
};
}
