// Generated macro for ConnectionParams (struct)
macro_rules! Depcrate_settingsConnectionParams {
() => {
// Module: crate::settings
// Provides: {"ConnectionParams"}
// Dependencies: {}
# [doc = " Combined configuration parameters required to establish a QUIC connection."] # [doc = ""] # [doc = " [`ConnectionParams`] aggregates the parameters required for all QUIC"] # [doc = " connections, regardless of whether it's a client- or server-side connection."] # [doc = " To construct them, either `ConnectionParams::new_server` or"] # [doc = " `ConnectionParams::new_client` must be used. The parameters can be modified"] # [doc = " freely after construction."] # [derive (Default)] # [non_exhaustive] pub struct ConnectionParams < 'a > { # [doc = " QUIC connection settings."] pub settings : QuicSettings , # [doc = " Optional TLS credentials to authenticate with."] pub tls_cert : Option < TlsCertificatePaths < 'a > > , # [doc = " Hooks to use for the connection."] pub hooks : Hooks , # [doc = " Set the session to attempt resumption."] pub session : Option < Vec < u8 > > , }
};
}
