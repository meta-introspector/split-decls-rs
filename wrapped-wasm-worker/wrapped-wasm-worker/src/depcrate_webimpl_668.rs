// Generated macro for impl_668 (impl)
macro_rules! Depcrate_webimpl_668 {
() => {
// Module: crate::web
// Provides: {"impl_668"}
// Dependencies: {}
impl < 'scope , T > ScopedJoinHandleExt < 'scope , T > for ScopedJoinHandle < 'scope , T > { fn join_async < 'handle > (& 'handle mut self) -> ScopedJoinHandleFuture < 'handle , 'scope , T > { ScopedJoinHandleFuture (self) } }
};
}
