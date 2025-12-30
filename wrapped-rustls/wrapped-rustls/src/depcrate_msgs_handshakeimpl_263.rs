// Generated macro for impl_263 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_263 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_263"}
// Dependencies: {}
impl Codec < '_ > for OcspCertificateStatusRequest { fn encode (& self , bytes : & mut Vec < u8 >) { CertificateStatusType :: OCSP . encode (bytes) ; self . responder_ids . encode (bytes) ; self . extensions . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { responder_ids : Vec :: read (r) ? , extensions : PayloadU16 :: read (r) ? , }) } }
};
}
