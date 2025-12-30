// Generated macro for ScopedJoinHandleFuture (struct)
macro_rules! Depcrate_webScopedJoinHandleFuture {
() => {
// Module: crate::web
// Provides: {"ScopedJoinHandleFuture"}
// Dependencies: {}
# [doc = " Waits for the associated thread to finish. See"] # [doc = " [`ScopedJoinHandleExt::join_async()`]."] # [must_use = "does nothing if not polled"] pub struct ScopedJoinHandleFuture < 'handle , 'scope , T > (& 'handle mut ScopedJoinHandle < 'scope , T >) ;
};
}
