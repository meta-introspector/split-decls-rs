// Generated macro for ServerConnection (struct)
macro_rules! Depcrate_clientServerConnection {
() => {
// Module: crate::client
// Provides: {"ServerConnection"}
// Dependencies: {}
# [doc = " A connection to an sccache server."] pub struct ServerConnection { # [doc = " A reader for the socket connected to the server."] reader : BufReader < Box < dyn Connection > > , # [doc = " A writer for the socket connected to the server."] writer : BufWriter < Box < dyn Connection > > , }
};
}
