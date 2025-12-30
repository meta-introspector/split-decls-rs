// Generated macro for impl_36 (impl)
macro_rules! Depcrate_clientimpl_36 {
() => {
// Module: crate::client
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (feature = "early-data")] impl < IO > TlsStream < IO > where IO : AsyncRead + AsyncWrite + Unpin , { fn poll_early_data (& mut self , cx : & mut Context < '_ >) { if self . early_waker . as_ref () . filter (| waker | cx . waker () . will_wake (waker)) . is_none () { self . early_waker = Some (cx . waker () . clone ()) ; } } }
};
}
