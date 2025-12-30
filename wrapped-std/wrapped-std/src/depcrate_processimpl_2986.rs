// Generated macro for impl_2986 (impl)
macro_rules! Depcrate_processimpl_2986 {
() => {
// Module: crate::process
// Provides: {"impl_2986"}
// Dependencies: {}
# [stable (feature = "command_access" , since = "1.57.0")] impl < 'a > Iterator for CommandEnvs < 'a > { type Item = (& 'a OsStr , Option < & 'a OsStr >) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
