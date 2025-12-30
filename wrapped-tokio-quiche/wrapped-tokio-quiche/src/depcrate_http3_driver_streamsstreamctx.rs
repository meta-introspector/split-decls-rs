// Generated macro for StreamCtx (struct)
macro_rules! Depcrate_http3_driver_streamsStreamCtx {
() => {
// Module: crate::http3::driver::streams
// Provides: {"StreamCtx"}
// Dependencies: {}
pub (crate) struct StreamCtx { # [doc = " Sends [`InboundFrame`]s to a local task, for example an `H3Body`."] pub (crate) send : Option < InboundFrameSender > , # [doc = " Receives [`OutboundFrame`]s from a local task."] pub (crate) recv : Option < OutboundFrameStream > , # [doc = " Stores the next [`OutboundFrame`] to write to the connection."] # [doc = " This is used as temporary storage when waiting for `recv`."] pub (crate) queued_frame : Option < OutboundFrame > , pub (crate) audit_stats : Arc < H3AuditStats > , # [doc = " Indicates the stream sent initial headers."] pub (crate) initial_headers_sent : bool , # [doc = " First time that a HEADERS frame was not fully flushed."] pub (crate) first_full_headers_flush_fail_time : Option < Instant > , # [doc = " Indicates the stream received fin or reset. No more data will be"] # [doc = " received."] pub (crate) fin_or_reset_recv : bool , # [doc = " Indicates the stream sent fin or reset. No more data will be sent."] pub (crate) fin_or_reset_sent : bool , # [doc = " The flow ID for proxying datagrams over this stream. If `None`,"] # [doc = " the stream has no associated DATAGRAM flow."] pub (crate) associated_dgram_flow_id : Option < u64 > , }
};
}
