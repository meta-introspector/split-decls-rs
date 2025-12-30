// Generated macro for impl_351 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_351 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_351"}
// Dependencies: {}
impl Codec < '_ > for ServerKeyExchangePayload { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: Known (x) => x . encode (bytes) , Self :: Unknown (x) => x . encode (bytes) , } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self :: Unknown (Payload :: read (r) . into_owned ())) } }
};
}
