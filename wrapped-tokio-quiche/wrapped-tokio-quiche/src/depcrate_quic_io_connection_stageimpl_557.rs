// Generated macro for impl_557 (impl)
macro_rules! Depcrate_quic_io_connection_stageimpl_557 {
() => {
// Module: crate::quic::io::connection_stage
// Provides: {"impl_557"}
// Dependencies: {}
impl ConnectionStage for RunningApplication { fn on_read < A : ApplicationOverQuic > (& mut self , received_packets : bool , qconn : & mut QuicheConnection , ctx : & mut ConnectionStageContext < A > ,) -> QuicResult < () > { if ctx . application . should_act () { if received_packets { ctx . application . process_reads (qconn) ? ; } if qconn . is_established () { ctx . application . process_writes (qconn) ? ; } } Ok (()) } }
};
}
