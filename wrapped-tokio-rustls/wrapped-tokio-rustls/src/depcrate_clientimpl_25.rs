// Generated macro for impl_25 (impl)
macro_rules! Depcrate_clientimpl_25 {
() => {
// Module: crate::client
// Provides: {"impl_25"}
// Dependencies: {}
impl TlsConnectorWithAlpn < '_ > { # [inline] pub fn connect < IO > (self , domain : ServerName < 'static > , stream : IO) -> Connect < IO > where IO : AsyncRead + AsyncWrite + Unpin , { self . inner . connect_impl (domain , stream , Some (self . alpn_protocols) , | _ | ()) } # [inline] pub fn connect_with < IO , F > (self , domain : ServerName < 'static > , stream : IO , f : F) -> Connect < IO > where IO : AsyncRead + AsyncWrite + Unpin , F : FnOnce (& mut ClientConnection) , { self . inner . connect_impl (domain , stream , Some (self . alpn_protocols) , f) } }
};
}
