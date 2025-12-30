// Generated macro for impl_7 (impl)
macro_rules! Depcrate_code_lockimpl_7 {
() => {
// Module: crate::code_lock
// Provides: {"impl_7"}
// Dependencies: {}
impl UniqueReentrantMutex { pub (crate) fn lock (& self) -> MutexGuardWrapper < '_ > { self . locks . serial () } pub (crate) fn start_parallel (& self) { self . locks . start_parallel () ; } pub (crate) fn end_parallel (& self) { self . locks . end_parallel () ; } # [cfg (test)] pub fn parallel_count (& self) -> u32 { self . locks . parallel_count () } # [cfg (test)] pub fn is_locked (& self) -> bool { self . locks . is_locked () } pub fn is_locked_by_current_thread (& self) -> bool { self . locks . is_locked_by_current_thread () } }
};
}
