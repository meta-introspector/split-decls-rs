// Generated macro for macro_220 (macro)
macro_rules! Depcrate_de_implsmacro_220 {
() => {
// Module: crate::de::impls
// Provides: {"macro_220"}
// Dependencies: {}
seq_impl ! (# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] HashSet < T : Eq + Hash , S : BuildHasher + Default >, seq , HashSet :: clear , HashSet :: with_capacity_and_hasher (size_hint :: cautious ::< T > (seq . size_hint ()) , S :: default ()) , HashSet :: reserve , HashSet :: insert) ;
};
}
