// Generated macro for impl_381 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_381 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'a > Codec < 'a > for HandshakeMessagePayload < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { self . payload_encode (bytes , Encoding :: Standard) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { Self :: read_version (r , ProtocolVersion :: TLSv1_2) } }
};
}
