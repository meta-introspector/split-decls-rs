// Generated macro for impl_652 (impl)
macro_rules! Depcrate_quic_io_workerimpl_652 {
() => {
// Module: crate::quic::io::worker
// Provides: {"impl_652"}
// Dependencies: {}
impl < Tx , M > IoWorker < Tx , M , RunningApplication > where Tx : DatagramSocketSend + Send , M : Metrics , { pub (crate) async fn run < A : ApplicationOverQuic > (mut self , mut qconn : QuicheConnection , mut ctx : ConnectionStageContext < A > ,) -> Closing < Tx , M , A > { if let Err (e) = self . conn_stage . on_read (true , & mut qconn , & mut ctx) { return Closing { params : self . into () , context : ctx , work_loop_result : Err (e) , qconn , } ; } ; let work_loop_result = self . work_loop (& mut qconn , & mut ctx) . await ; Closing { params : self . into () , context : ctx , work_loop_result , qconn , } } }
};
}
