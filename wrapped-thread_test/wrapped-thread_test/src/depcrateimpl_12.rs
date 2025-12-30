// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Drop for DtorNotifier { fn drop (& mut self) { TLS_DESTRUCTOR_RAN . store (true , Ordering :: SeqCst) ; } }
};
}
