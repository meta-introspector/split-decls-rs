// Generated macro for InboundFrame (enum)
macro_rules! Depcrate_http3_driverInboundFrame {
() => {
// Module: crate::http3::driver
// Provides: {"InboundFrame"}
// Dependencies: {}
# [doc = " An [`InboundFrame`] is a data frame that was received from the peer over a"] # [doc = " [`quiche::h3::Connection`]. This is used by peers to send body or datagrams"] # [doc = " to the local task."] # [derive (Debug)] pub enum InboundFrame { # [doc = " Request body/CONNECT upstream data plus FIN flag."] Body (PooledBuf , bool) , # [doc = " CONNECT-UDP (DATAGRAM) upstream data."] Datagram (PooledDgram) , }
};
}
