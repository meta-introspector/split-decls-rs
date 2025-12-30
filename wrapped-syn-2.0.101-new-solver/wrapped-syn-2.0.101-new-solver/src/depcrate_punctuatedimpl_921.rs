// Generated macro for impl_921 (impl)
macro_rules! Depcrate_punctuatedimpl_921 {
() => {
// Module: crate::punctuated
// Provides: {"impl_921"}
// Dependencies: {}
# [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Clone for Pair < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { match self { Pair :: Punctuated (t , p) => Pair :: Punctuated (t . clone () , p . clone ()) , Pair :: End (t) => Pair :: End (t . clone ()) , } } }
};
}
