// Generated macro for impl_373 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_373 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_373"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CertificateStatus < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { CertificateStatusType :: OCSP . encode (bytes) ; self . ocsp_response . encode (bytes) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let typ = CertificateStatusType :: read (r) ? ; match typ { CertificateStatusType :: OCSP => Ok (Self { ocsp_response : PayloadU24 :: read (r) ? , }) , _ => Err (InvalidMessage :: InvalidCertificateStatusType) , } } }
};
}
