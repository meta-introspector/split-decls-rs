// Generated macro for impl_3690 (impl)
macro_rules! Depcrate_sys_process_envimpl_3690 {
() => {
// Module: crate::sys::process::env
// Provides: {"impl_3690"}
// Dependencies: {}
impl < 'a > Iterator for CommandEnvs < 'a > { type Item = (& 'a OsStr , Option < & 'a OsStr >) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (key , value) | (key . as_ref () , value . as_deref ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
