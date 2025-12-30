// Generated macro for impl_98 (impl)
macro_rules! Depcrate_sync_mpscimpl_98 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_98"}
// Dependencies: {}
impl < T > Clone for PollSender < T > { # [doc = " Clones this `PollSender`."] # [doc = ""] # [doc = " The resulting `PollSender` will have an initial state identical to calling `PollSender::new`."] fn clone (& self) -> PollSender < T > { let (sender , state) = match self . sender . clone () { Some (sender) => (Some (sender . clone ()) , State :: Idle (sender)) , None => (None , State :: Closed) , } ; Self { sender , state , acquire : PollSenderFuture :: empty () , } } }
};
}
