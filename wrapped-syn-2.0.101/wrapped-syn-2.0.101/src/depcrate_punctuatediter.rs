// Generated macro for Iter (struct)
macro_rules! Depcrate_punctuatedIter {
() => {
// Module: crate::punctuated
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over borrowed values of type `&T`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct Iter < 'a , T : 'a > { inner : Box < NoDrop < dyn IterTrait < 'a , T > + 'a > > , }
};
}
