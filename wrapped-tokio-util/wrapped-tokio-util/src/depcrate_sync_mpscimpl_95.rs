// Generated macro for impl_95 (impl)
macro_rules! Depcrate_sync_mpscimpl_95 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_95"}
// Dependencies: {}
impl < T > PollSenderFuture < T > { # [doc = " Create with an empty inner future with no `Send` bound."] fn empty () -> Self { Self (ReusableBoxFuture :: new (async { unreachable ! () })) } }
};
}
