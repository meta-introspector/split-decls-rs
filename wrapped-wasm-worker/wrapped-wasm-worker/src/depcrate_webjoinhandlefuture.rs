// Generated macro for JoinHandleFuture (struct)
macro_rules! Depcrate_webJoinHandleFuture {
() => {
// Module: crate::web
// Provides: {"JoinHandleFuture"}
// Dependencies: {}
# [doc = " Waits for the associated thread to finish. See"] # [doc = " [`JoinHandleExt::join_async()`]."] # [must_use = "does nothing if not polled"] pub struct JoinHandleFuture < 'handle , T > (& 'handle mut JoinHandle < T >) ;
};
}
