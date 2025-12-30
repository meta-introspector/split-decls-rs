// Generated macro for InnerFuture (type)
macro_rules! Depcrate_sync_mpscInnerFuture {
() => {
// Module: crate::sync::mpsc
// Provides: {"InnerFuture"}
// Dependencies: {}
type InnerFuture < 'a , T > = ReusableBoxFuture < 'a , Result < OwnedPermit < T > , PollSendError < T > > > ;
};
}
