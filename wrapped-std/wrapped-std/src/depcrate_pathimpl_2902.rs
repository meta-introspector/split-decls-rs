// Generated macro for impl_2902 (impl)
macro_rules! Depcrate_pathimpl_2902 {
() => {
// Module: crate::path
// Provides: {"impl_2902"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialOrd for Path { # [inline] fn partial_cmp (& self , other : & Path) -> Option < cmp :: Ordering > { Some (compare_components (self . components () , other . components ())) } }
};
}
