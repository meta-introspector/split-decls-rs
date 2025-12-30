// Generated macro for impl_325 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_325 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_325"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CertificatePayloadTls13 < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { self . context . encode (bytes) ; self . entries . encode (bytes) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { Ok (Self { context : PayloadU8 :: read (r) ? , entries : Vec :: read (r) ? , }) } }
};
}
