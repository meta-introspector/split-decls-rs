// Generated macro for impl_914 (impl)
macro_rules! Depcrate_util_readyimpl_914 {
() => {
// Module: crate::util::ready
// Provides: {"impl_914"}
// Dependencies: {}
impl < T , Request > Future for ReadyOneshot < T , Request > where T : Service < Request > , { type Output = Result < T , T :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ready ! (self . inner . as_mut () . expect ("poll after Poll::Ready") . poll_ready (cx)) ? ; Poll :: Ready (Ok (self . inner . take () . expect ("poll after Poll::Ready"))) } }
};
}
