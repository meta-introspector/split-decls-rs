// Generated macro for ScopeIntoJoinFuture (struct)
macro_rules! Depcrate_webScopeIntoJoinFuture {
() => {
// Module: crate::web
// Provides: {"ScopeIntoJoinFuture"}
// Dependencies: {}
# [doc = " Poll to completion to get a [`ScopeJoinFuture`]. See"] # [doc = " [`ScopeFuture::into_wait()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Keep in mind that if dropped it will block, or spinloop if blocking is not"] # [doc = " supported on this thread (see [`has_block_support()`]), until all threads"] # [doc = " are joined but does not continue polling the [`Future`] passed into"] # [doc = " [`scope_async()`]."] # [must_use = "will block until all spawned threads are finished if not polled to completion"] # [cfg_attr (all (target_family = "wasm" , target_os = "unknown") , pin_project)] pub struct ScopeIntoJoinFuture < 'scope , 'env , F , T > (# [cfg_attr (all (target_family = "wasm" , target_os = "unknown") , pin)] ScopeFuture < 'scope , 'env , F , T > ,) ;
};
}
