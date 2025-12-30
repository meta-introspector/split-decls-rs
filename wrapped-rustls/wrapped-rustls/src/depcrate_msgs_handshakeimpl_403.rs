// Generated macro for impl_403 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_403 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_403"}
// Dependencies: {}
impl Codec < '_ > for ServerEncryptedClientHello { fn encode (& self , bytes : & mut Vec < u8 >) { self . retry_configs . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { retry_configs : Vec :: < EchConfigPayload > :: read (r) ? , }) } }
};
}
