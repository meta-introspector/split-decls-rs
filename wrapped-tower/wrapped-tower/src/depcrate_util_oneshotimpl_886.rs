// Generated macro for impl_886 (impl)
macro_rules! Depcrate_util_oneshotimpl_886 {
() => {
// Module: crate::util::oneshot
// Provides: {"impl_886"}
// Dependencies: {}
impl < S , Req > Future for Oneshot < S , Req > where S : Service < Req > , { type Output = Result < S :: Response , S :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state . as_mut () . project () { StateProj :: NotReady { svc , req } => { ready ! (svc . poll_ready (cx)) ? ; let f = svc . call (req . take () . expect ("already called")) ; this . state . set (State :: called (f)) ; } StateProj :: Called { fut } => { let res = ready ! (fut . poll (cx)) ? ; this . state . set (State :: Done) ; return Poll :: Ready (Ok (res)) ; } StateProj :: Done => panic ! ("polled after complete") , } } } }
};
}
