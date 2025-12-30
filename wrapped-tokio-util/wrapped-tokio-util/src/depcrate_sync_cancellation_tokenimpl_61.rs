// Generated macro for impl_61 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_61 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_61"}
// Dependencies: {}
impl Clone for CancellationToken { # [doc = " Creates a clone of the [`CancellationToken`] which will get cancelled"] # [doc = " whenever the current token gets cancelled, and vice versa."] fn clone (& self) -> Self { tree_node :: increase_handle_refcount (& self . inner) ; CancellationToken { inner : self . inner . clone () , } } }
};
}
