// Generated macro for impl_2903 (impl)
macro_rules! Depcrate_pathimpl_2903 {
() => {
// Module: crate::path
// Provides: {"impl_2903"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Ord for Path { # [inline] fn cmp (& self , other : & Path) -> cmp :: Ordering { compare_components (self . components () , other . components ()) } }
};
}
