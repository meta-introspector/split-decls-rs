// Generated macro for impl_2883 (impl)
macro_rules! Depcrate_pathimpl_2883 {
() => {
// Module: crate::path
// Provides: {"impl_2883"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Ord for PathBuf { # [inline] fn cmp (& self , other : & PathBuf) -> cmp :: Ordering { compare_components (self . components () , other . components ()) } }
};
}
