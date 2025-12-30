// Generated macro for impl_2882 (impl)
macro_rules! Depcrate_pathimpl_2882 {
() => {
// Module: crate::path
// Provides: {"impl_2882"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialOrd for PathBuf { # [inline] fn partial_cmp (& self , other : & PathBuf) -> Option < cmp :: Ordering > { Some (compare_components (self . components () , other . components ())) } }
};
}
