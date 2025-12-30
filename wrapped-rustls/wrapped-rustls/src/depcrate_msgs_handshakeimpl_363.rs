// Generated macro for impl_363 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_363 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_363"}
// Dependencies: {}
impl Codec < '_ > for CertificateRequestPayloadTls13 { fn encode (& self , bytes : & mut Vec < u8 >) { self . context . encode (bytes) ; self . extensions . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let context = PayloadU8 :: read (r) ? ; let extensions = CertificateRequestExtensions :: read (r) ? ; Ok (Self { context , extensions , }) } }
};
}
