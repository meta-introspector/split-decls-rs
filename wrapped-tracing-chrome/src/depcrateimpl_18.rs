// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl Drop for FlushGuard { fn drop (& mut self) { if let Some (handle) = self . handle . take () { let _ignored = self . sender . send (Message :: Drop) ; if handle . join () . is_err () { eprintln ! ("tracing_chrome: Trace writing thread panicked.") ; } } } }
};
}
