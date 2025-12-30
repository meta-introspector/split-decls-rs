// Generated macro for ConnCloseReceiver (struct)
macro_rules! Depcrate_quic_rawConnCloseReceiver {
() => {
// Module: crate::quic::raw
// Provides: {"ConnCloseReceiver"}
// Dependencies: {}
# [doc = " Pollable receiver for `connection closed` notifications from a QUIC"] # [doc = " connection."] # [doc = ""] # [doc = " This receiver also fires if the corresponding sender has been dropped"] # [doc = " without a `CONNECTION_CLOSE` frame on the connection."] pub struct ConnCloseReceiver (mpsc :: UnboundedReceiver < ConnectionMapCommand >) ;
};
}
