// Generated macro for impl_549 (impl)
macro_rules! Depcrate_envimpl_549 {
() => {
// Module: crate::env
// Provides: {"impl_549"}
// Dependencies: {}
# [stable (feature = "env" , since = "1.0.0")] impl < 'a > Iterator for SplitPaths < 'a > { type Item = PathBuf ; fn next (& mut self) -> Option < PathBuf > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
