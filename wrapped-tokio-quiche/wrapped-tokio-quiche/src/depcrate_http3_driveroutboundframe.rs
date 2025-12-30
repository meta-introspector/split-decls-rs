// Generated macro for OutboundFrame (enum)
macro_rules! Depcrate_http3_driverOutboundFrame {
() => {
// Module: crate::http3::driver
// Provides: {"OutboundFrame"}
// Dependencies: {}
# [doc = " An [`OutboundFrame`] is a data frame that should be sent from a local task"] # [doc = " to a peer over a [`quiche::h3::Connection`]."] # [doc = ""] # [doc = " This is used, for example, to send response body data to a peer, or proxied"] # [doc = " UDP datagrams."] # [derive (Debug)] pub enum OutboundFrame { # [doc = " Response headers to be sent to the peer, with optional priority."] Headers (Vec < h3 :: Header > , Option < quiche :: h3 :: Priority >) , # [doc = " Response body/CONNECT downstream data plus FIN flag."] # [cfg (feature = "zero-copy")] Body (crate :: buf_factory :: QuicheBuf , bool) , # [doc = " Response body/CONNECT downstream data plus FIN flag."] # [cfg (not (feature = "zero-copy"))] Body (PooledBuf , bool) , # [doc = " CONNECT-UDP (DATAGRAM) downstream data plus flow ID."] Datagram (PooledDgram , u64) , # [doc = " Close the stream with a trailers, with optional priority."] Trailers (Vec < h3 :: Header > , Option < quiche :: h3 :: Priority >) , # [doc = " An error encountered when serving the request. Stream should be closed."] PeerStreamError , # [doc = " DATAGRAM flow explicitly closed."] FlowShutdown { flow_id : u64 , stream_id : u64 } , }
};
}
