// Generated macro for IncomingH3Headers (struct)
macro_rules! Depcrate_http3_driverIncomingH3Headers {
() => {
// Module: crate::http3::driver
// Provides: {"IncomingH3Headers"}
// Dependencies: {}
# [doc = " HTTP/3 headers that were received on a stream."] # [doc = ""] # [doc = " `recv` is used to read the message body, while `send` is used to transmit"] # [doc = " data back to the peer."] pub struct IncomingH3Headers { # [doc = " Stream ID of the frame."] pub stream_id : u64 , # [doc = " The actual [`h3::Header`]s which were received."] pub headers : Vec < h3 :: Header > , # [doc = " An [`OutboundFrameSender`] for streaming body data to the peer. For"] # [doc = " [ClientH3Driver], note that the request body can also be passed a"] # [doc = " cloned sender via [`NewClientRequest`]."] pub send : OutboundFrameSender , # [doc = " An [`InboundFrameStream`] of body data received from the peer."] pub recv : InboundFrameStream , # [doc = " Whether there is a body associated with the incoming headers."] pub read_fin : bool , # [doc = " Handle to the [`H3AuditStats`] for the message's stream."] pub h3_audit_stats : Arc < H3AuditStats > , }
};
}
