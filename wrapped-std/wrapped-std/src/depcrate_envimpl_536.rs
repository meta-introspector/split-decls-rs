// Generated macro for impl_536 (impl)
macro_rules! Depcrate_envimpl_536 {
() => {
// Module: crate::env
// Provides: {"impl_536"}
// Dependencies: {}
# [stable (feature = "env" , since = "1.0.0")] impl Iterator for VarsOs { type Item = (OsString , OsString) ; fn next (& mut self) -> Option < (OsString , OsString) > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
