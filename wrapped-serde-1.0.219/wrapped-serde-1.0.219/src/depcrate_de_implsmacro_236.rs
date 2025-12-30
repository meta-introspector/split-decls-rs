// Generated macro for macro_236 (macro)
macro_rules! Depcrate_de_implsmacro_236 {
() => {
// Module: crate::de::impls
// Provides: {"macro_236"}
// Dependencies: {}
map_impl ! { # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] HashMap < K : Eq + Hash , V , S : BuildHasher + Default >, map , HashMap :: with_capacity_and_hasher (size_hint :: cautious ::< (K , V) > (map . size_hint ()) , S :: default ()) , }
};
}
