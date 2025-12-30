// Generated macro for impl_2829 (impl)
macro_rules! Depcrate_pathimpl_2829 {
() => {
// Module: crate::path
// Provides: {"impl_2829"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > Iterator for Iter < 'a > { type Item = & 'a OsStr ; # [inline] fn next (& mut self) -> Option < & 'a OsStr > { self . inner . next () . map (Component :: as_os_str) } }
};
}
