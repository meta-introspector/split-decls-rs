// Generated macro for empty_punctuated_iter_mut (function)
macro_rules! Depcrate_punctuatedempty_punctuated_iter_mut {
() => {
// Module: crate::punctuated
// Provides: {"empty_punctuated_iter_mut"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] pub (crate) fn empty_punctuated_iter_mut < 'a , T > () -> IterMut < 'a , T > { IterMut { inner : Box :: new (NoDrop :: new (iter :: empty ())) , } }
};
}
