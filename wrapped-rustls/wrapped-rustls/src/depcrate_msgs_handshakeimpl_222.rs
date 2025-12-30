// Generated macro for impl_222 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_222 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_222"}
// Dependencies: {}
impl PartialEq for SessionId { fn eq (& self , other : & Self) -> bool { if self . len != other . len { return false ; } let mut diff = 0u8 ; for i in 0 .. self . len { diff |= self . data [i] ^ other . data [i] ; } diff == 0u8 } }
};
}
