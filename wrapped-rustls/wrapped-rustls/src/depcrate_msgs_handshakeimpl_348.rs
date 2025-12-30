// Generated macro for impl_348 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_348 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_348"}
// Dependencies: {}
impl ServerKeyExchange { pub (crate) fn encode (& self , buf : & mut Vec < u8 >) { self . params . encode (buf) ; self . dss . encode (buf) ; } }
};
}
