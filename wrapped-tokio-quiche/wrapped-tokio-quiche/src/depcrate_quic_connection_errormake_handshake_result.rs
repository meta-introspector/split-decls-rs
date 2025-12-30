// Generated macro for make_handshake_result (function)
macro_rules! Depcrate_quic_connection_errormake_handshake_result {
() => {
// Module: crate::quic::connection::error
// Provides: {"make_handshake_result"}
// Dependencies: {}
# [doc = " Derives a [`std::io::Result`] from `IoWorker::handshake`'s result without"] # [doc = " taking ownership of the original [`Result`]."] pub (crate) fn make_handshake_result < T > (res : & QuicResult < () >) -> io :: Result < T > { let Err (err) = res else { return Err (io :: Error :: other ("Handshake transitioned to Closing without error" ,)) ; } ; if let Some (hs_err) = err . downcast_ref :: < HandshakeError > () { Err (hs_err . clone () . into ()) } else if let Some (quiche_err) = err . downcast_ref :: < quiche :: Error > () { Err (io :: Error :: other (* quiche_err)) } else { let data_fmt = format ! ("unexpected handshake error: {err}") ; Err (io :: Error :: other (data_fmt)) } }
};
}
