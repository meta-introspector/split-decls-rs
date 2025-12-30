// Generated macro for impl_221 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_221 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_221"}
// Dependencies: {}
impl fmt :: Debug for SessionId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { super :: base :: hex (f , & self . data [.. self . len]) } }
};
}
