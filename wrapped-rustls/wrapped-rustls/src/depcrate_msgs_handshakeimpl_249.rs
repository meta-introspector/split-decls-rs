// Generated macro for impl_249 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_249 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_249"}
// Dependencies: {}
impl KeyShareEntry { pub (crate) fn new (group : NamedGroup , payload : impl Into < Vec < u8 > >) -> Self { Self { group , payload : PayloadU16 :: new (payload . into ()) , } } }
};
}
