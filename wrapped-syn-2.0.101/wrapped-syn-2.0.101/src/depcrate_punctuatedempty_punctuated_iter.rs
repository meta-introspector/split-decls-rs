// Generated macro for empty_punctuated_iter (function)
macro_rules! Depcrate_punctuatedempty_punctuated_iter {
() => {
// Module: crate::punctuated
// Provides: {"empty_punctuated_iter"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] pub (crate) fn empty_punctuated_iter < 'a , T > () -> Iter < 'a , T > { Iter { inner : Box :: new (NoDrop :: new (iter :: empty ())) , } }
};
}
