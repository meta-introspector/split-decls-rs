// Generated macro for impl_376 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_376 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_376"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CompressedCertificatePayload < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { self . alg . encode (bytes) ; codec :: u24 (self . uncompressed_len) . encode (bytes) ; self . compressed . encode (bytes) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { Ok (Self { alg : CertificateCompressionAlgorithm :: read (r) ? , uncompressed_len : codec :: u24 :: read (r) ? . 0 , compressed : PayloadU24 :: read (r) ? , }) } }
};
}
