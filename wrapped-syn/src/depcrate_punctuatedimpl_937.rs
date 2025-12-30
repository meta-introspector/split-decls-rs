// Generated macro for impl_937 (impl)
macro_rules! Depcrate_punctuatedimpl_937 {
() => {
// Module: crate::punctuated
// Provides: {"impl_937"}
// Dependencies: {}
# [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl < T , P > Pair < & T , & P > { pub fn cloned (self) -> Pair < T , P > where T : Clone , P : Clone , { match self { Pair :: Punctuated (t , p) => Pair :: Punctuated (t . clone () , p . clone ()) , Pair :: End (t) => Pair :: End (t . clone ()) , } } }
};
}
