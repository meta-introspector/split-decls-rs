// Generated macro for impl_555 (impl)
macro_rules! Depcrate_quic_io_connection_stageimpl_555 {
() => {
// Module: crate::quic::io::connection_stage
// Provides: {"impl_555"}
// Dependencies: {}
impl ConnectionStage for Handshake { fn on_flush < A : ApplicationOverQuic > (& mut self , qconn : & mut QuicheConnection , _ctx : & mut ConnectionStageContext < A > ,) -> ControlFlow < QuicResult < () > > { if qconn . is_established () { ControlFlow :: Break (Ok (())) } else { ControlFlow :: Continue (()) } } fn wait_deadline (& mut self) -> Option < Instant > { self . handshake_info . deadline () } fn post_wait (& self , qconn : & mut QuicheConnection ,) -> ControlFlow < QuicResult < () > > { match self . check_handshake_timeout_expired (qconn) { Ok (_) => ControlFlow :: Continue (()) , Err (e) => ControlFlow :: Break (Err (e)) , } } }
};
}
