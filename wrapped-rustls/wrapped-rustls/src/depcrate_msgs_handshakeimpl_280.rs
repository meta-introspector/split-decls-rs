// Generated macro for impl_280 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_280 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_280"}
// Dependencies: {}
impl TransportParameters < '_ > { pub (crate) fn into_owned (self) -> TransportParameters < 'static > { match self { Self :: Quic (v) => TransportParameters :: Quic (v . into_owned ()) , } } }
};
}
