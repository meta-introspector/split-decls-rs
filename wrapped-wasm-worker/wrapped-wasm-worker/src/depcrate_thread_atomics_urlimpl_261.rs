// Generated macro for impl_261 (impl)
macro_rules! Depcrate_thread_atomics_urlimpl_261 {
() => {
// Module: crate::thread::atomics::url
// Provides: {"impl_261"}
// Dependencies: {}
impl Drop for ScriptUrl { fn drop (& mut self) { Url :: revoke_object_url (& self . 0) . expect ("`URL.revokeObjectURL()` should never throw") ; } }
};
}
