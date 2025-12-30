// Generated macro for impl_2841 (impl)
macro_rules! Depcrate_pathimpl_2841 {
() => {
// Module: crate::path
// Provides: {"impl_2841"}
// Dependencies: {}
# [stable (feature = "path_ancestors" , since = "1.28.0")] impl < 'a > Iterator for Ancestors < 'a > { type Item = & 'a Path ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let next = self . next ; self . next = next . and_then (Path :: parent) ; next } }
};
}
