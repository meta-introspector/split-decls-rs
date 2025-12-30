// Generated macro for impl_2983 (impl)
macro_rules! Depcrate_processimpl_2983 {
() => {
// Module: crate::process
// Provides: {"impl_2983"}
// Dependencies: {}
# [stable (feature = "command_access" , since = "1.57.0")] impl < 'a > Iterator for CommandArgs < 'a > { type Item = & 'a OsStr ; fn next (& mut self) -> Option < & 'a OsStr > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
