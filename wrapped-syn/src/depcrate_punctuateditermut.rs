// Generated macro for IterMut (struct)
macro_rules! Depcrate_punctuatedIterMut {
() => {
// Module: crate::punctuated
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator over mutably borrowed values of type `&mut T`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct IterMut < 'a , T : 'a > { inner : Box < NoDrop < dyn IterMutTrait < 'a , T , Item = & 'a mut T > + 'a > > , }
};
}
