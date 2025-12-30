// Generated macro for ScopeJoinFuture (struct)
macro_rules! Depcrate_webScopeJoinFuture {
() => {
// Module: crate::web
// Provides: {"ScopeJoinFuture"}
// Dependencies: {}
# [doc = " Waits for the associated scope to finish. See [`ScopeFuture::into_wait()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Keep in mind that if dropped it will block, or spinloop if blocking is not"] # [doc = " supported on this thread (see [`has_block_support()`]), until all threads"] # [doc = " are joined."] # [must_use = "will block until all spawned threads are finished if not polled to completion"] pub struct ScopeJoinFuture < 'scope , 'env , T > (ScopeFuture < 'scope , 'env , Ready < T > , T >) ;
};
}
