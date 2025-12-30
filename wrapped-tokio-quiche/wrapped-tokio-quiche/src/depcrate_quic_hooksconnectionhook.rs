// Generated macro for ConnectionHook (trait)
macro_rules! Depcrate_quic_hooksConnectionHook {
() => {
// Module: crate::quic::hooks
// Provides: {"ConnectionHook"}
// Dependencies: {}
# [doc = " A set of hooks executed at the level of a [quiche::Connection]."] pub trait ConnectionHook { # [doc = " Constructs an optional [`SslContextBuilder`]."] # [doc = ""] # [doc = " This method allows full customization of quiche's SSL context, for"] # [doc = " example to specify async callbacks during the QUIC handshake. It is"] # [doc = " called once per socket during initial setup, and then reused across"] # [doc = " all connections on that socket."] # [doc = ""] # [doc = " Only called if both the hook and [`TlsCertificatePaths`] are set in"] # [doc = " [`ConnectionParams`](crate::ConnectionParams)."] fn create_custom_ssl_context_builder (& self , settings : TlsCertificatePaths < '_ > ,) -> Option < SslContextBuilder > ; }
};
}
