// Generated macro for ConnectionStageContext (struct)
macro_rules! Depcrate_quic_io_connection_stageConnectionStageContext {
() => {
// Module: crate::quic::io::connection_stage
// Provides: {"ConnectionStageContext"}
// Dependencies: {}
# [doc = " Global context shared across all [ConnectionStage]s for a given connection"] pub struct ConnectionStageContext < A > { pub in_pkt : Option < Incoming > , pub application : A , pub incoming_pkt_receiver : mpsc :: Receiver < Incoming > , pub stats : QuicConnectionStatsShared , }
};
}
