// Generated macro for impl_530 (impl)
macro_rules! Depcrate_quic_connectionimpl_530 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_530"}
// Dependencies: {}
impl QuicCommand { # [doc = " Consume the command and perform its operation on `qconn`."] # [doc = ""] # [doc = " This method should be called by [`ApplicationOverQuic`] implementations"] # [doc = " when they receive a [`QuicCommand`] to execute."] pub fn execute (self , qconn : & mut QuicheConnection) { match self { Self :: ConnectionClose (behavior) => { let ConnectionShutdownBehaviour { send_application_close , error_code , reason , } = behavior ; let _ = qconn . close (send_application_close , error_code , & reason) ; } , Self :: Custom (f) => { (f) (qconn) ; } , Self :: Stats (callback) => { let stats_pair = QuicConnectionStats :: from_conn (qconn) ; (callback) (stats_pair . as_socket_stats ()) ; } , } } }
};
}
