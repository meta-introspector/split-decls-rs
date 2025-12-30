// Generated macro for impl_320 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_320 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_320"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CertificateEntry < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { self . cert . encode (bytes) ; self . extensions . encode (bytes) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { Ok (Self { cert : CertificateDer :: read (r) ? , extensions : CertificateExtensions :: read (r) ? . into_owned () , }) } }
};
}
