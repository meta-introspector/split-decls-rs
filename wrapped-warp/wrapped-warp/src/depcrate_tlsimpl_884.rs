// Generated macro for impl_884 (impl)
macro_rules! Depcrate_tlsimpl_884 {
() => {
// Module: crate::tls
// Provides: {"impl_884"}
// Dependencies: {}
impl Accept for TlsAcceptor { type Conn = TlsStream ; type Error = io :: Error ; fn poll_accept (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Self :: Conn , Self :: Error > > > { let pin = self . get_mut () ; match ready ! (Pin :: new (& mut pin . incoming) . poll_accept (cx)) { Some (Ok (sock)) => Poll :: Ready (Some (Ok (TlsStream :: new (sock , pin . config . clone ())))) , Some (Err (e)) => Poll :: Ready (Some (Err (e))) , None => Poll :: Ready (None) , } } }
};
}
