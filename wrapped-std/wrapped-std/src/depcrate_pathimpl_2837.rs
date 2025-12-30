// Generated macro for impl_2837 (impl)
macro_rules! Depcrate_pathimpl_2837 {
() => {
// Module: crate::path
// Provides: {"impl_2837"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > PartialOrd for Components < 'a > { # [inline] fn partial_cmp (& self , other : & Components < 'a >) -> Option < cmp :: Ordering > { Some (compare_components (self . clone () , other . clone ())) } }
};
}
