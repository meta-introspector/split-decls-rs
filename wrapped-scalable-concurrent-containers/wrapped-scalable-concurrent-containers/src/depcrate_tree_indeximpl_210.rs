// Generated macro for impl_210 (impl)
macro_rules! Depcrate_tree_indeximpl_210 {
() => {
// Module: crate::tree_index
// Provides: {"impl_210"}
// Dependencies: {}
impl < K , V > Debug for TreeIndex < K , V > where K : 'static + Clone + Debug + Ord , V : 'static + Clone + Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let guard = Guard :: new () ; f . debug_map () . entries (self . iter (& guard)) . finish () } }
};
}
