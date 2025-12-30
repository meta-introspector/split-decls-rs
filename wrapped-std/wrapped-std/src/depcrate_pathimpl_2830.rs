// Generated macro for impl_2830 (impl)
macro_rules! Depcrate_pathimpl_2830 {
() => {
// Module: crate::path
// Provides: {"impl_2830"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > DoubleEndedIterator for Iter < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a OsStr > { self . inner . next_back () . map (Component :: as_os_str) } }
};
}
