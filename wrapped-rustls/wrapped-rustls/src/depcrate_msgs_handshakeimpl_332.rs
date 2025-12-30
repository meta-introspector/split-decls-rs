// Generated macro for impl_332 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_332 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_332"}
// Dependencies: {}
impl ClientKeyExchangeParams { pub (crate) fn pub_key (& self) -> & [u8] { match self { Self :: Ecdh (ecdh) => & ecdh . public . 0 , Self :: Dh (dh) => & dh . public . 0 , } } pub (crate) fn encode (& self , buf : & mut Vec < u8 >) { match self { Self :: Ecdh (ecdh) => ecdh . encode (buf) , Self :: Dh (dh) => dh . encode (buf) , } } }
};
}
