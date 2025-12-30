// Generated macro for Connect (struct)
macro_rules! Depcrate_clientConnect {
() => {
// Module: crate::client
// Provides: {"Connect"}
// Dependencies: {}
# [doc = " Future returned from `TlsConnector::connect` which will resolve"] # [doc = " once the connection handshake has finished."] pub struct Connect < IO > (MidHandshake < TlsStream < IO > >) ;
};
}
