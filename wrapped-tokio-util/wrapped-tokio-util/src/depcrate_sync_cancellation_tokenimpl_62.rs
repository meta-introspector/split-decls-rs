// Generated macro for impl_62 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_62 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_62"}
// Dependencies: {}
impl Drop for CancellationToken { fn drop (& mut self) { tree_node :: decrease_handle_refcount (& self . inner) ; } }
};
}
