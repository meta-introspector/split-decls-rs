// Generated macro for impl_209 (impl)
macro_rules! Depcrate_tree_indeximpl_209 {
() => {
// Module: crate::tree_index
// Provides: {"impl_209"}
// Dependencies: {}
impl < K , V > Clone for TreeIndex < K , V > where K : 'static + Clone + Ord , V : 'static + Clone , { # [inline] fn clone (& self) -> Self { let self_clone = Self :: default () ; for (k , v) in self . iter (& Guard :: new ()) { let _result : Result < () , (K , V) > = self_clone . insert_sync (k . clone () , v . clone ()) ; } self_clone } }
};
}
