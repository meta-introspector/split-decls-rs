// Generated macro for ScopeFuture (struct)
macro_rules! Depcrate_webScopeFuture {
() => {
// Module: crate::web
// Provides: {"ScopeFuture"}
// Dependencies: {}
# [doc = " Waits for the associated scope to finish. See [`scope_async()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Keep in mind that if dropped it will block, or spinloop if blocking is not"] # [doc = " supported on this thread (see [`has_block_support()`]), until all threads"] # [doc = " are joined but does not continue polling the passed [`Future`]."] # [must_use = "will block until all spawned threads are finished if not polled to completion"] # [cfg_attr (all (target_family = "wasm" , target_os = "unknown") , pin_project)] pub struct ScopeFuture < 'scope , 'env , F , T > (# [cfg_attr (all (target_family = "wasm" , target_os = "unknown") , pin)] thread :: ScopeFuture < 'scope , 'env , F , T > ,) ;
};
}
