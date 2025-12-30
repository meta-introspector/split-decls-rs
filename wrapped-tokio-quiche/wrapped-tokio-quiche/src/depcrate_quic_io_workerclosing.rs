// Generated macro for Closing (struct)
macro_rules! Depcrate_quic_io_workerClosing {
() => {
// Module: crate::quic::io::worker
// Provides: {"Closing"}
// Dependencies: {}
pub (crate) struct Closing < Tx , M , A > { pub (crate) params : IoWorkerParams < Tx , M > , pub (crate) context : ConnectionStageContext < A > , pub (crate) work_loop_result : QuicResult < () > , pub (crate) qconn : QuicheConnection , }
};
}
