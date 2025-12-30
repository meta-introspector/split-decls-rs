// Generated macro for impl_938 (impl)
macro_rules! Depcrate_punctuatedimpl_938 {
() => {
// Module: crate::punctuated
// Provides: {"impl_938"}
// Dependencies: {}
# [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Clone for Pair < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { match self { Pair :: Punctuated (t , p) => Pair :: Punctuated (t . clone () , p . clone ()) , Pair :: End (t) => Pair :: End (t . clone ()) , } } }
};
}
