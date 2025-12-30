// Generated macro for impl_213 (impl)
macro_rules! Depcrate_tree_indeximpl_213 {
() => {
// Module: crate::tree_index
// Provides: {"impl_213"}
// Dependencies: {}
impl < K , V > PartialEq for TreeIndex < K , V > where K : 'static + Clone + Ord , V : 'static + Clone + PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { let guard = Guard :: new () ; Iterator :: eq (self . iter (& guard) , other . iter (& guard)) } }
};
}
