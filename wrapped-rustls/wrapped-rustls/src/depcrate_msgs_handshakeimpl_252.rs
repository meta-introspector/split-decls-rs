// Generated macro for impl_252 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_252 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_252"}
// Dependencies: {}
impl PresharedKeyIdentity { pub (crate) fn new (id : Vec < u8 > , age : u32) -> Self { Self { identity : PayloadU16 :: new (id) , obfuscated_ticket_age : age , } } }
};
}
