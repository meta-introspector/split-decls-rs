// Generated macro for impl_377 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_377 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_377"}
// Dependencies: {}
impl CompressedCertificatePayload < '_ > { fn into_owned (self) -> CompressedCertificatePayload < 'static > { CompressedCertificatePayload { compressed : self . compressed . into_owned () , .. self } } pub (crate) fn as_borrowed (& self) -> CompressedCertificatePayload < '_ > { CompressedCertificatePayload { alg : self . alg , uncompressed_len : self . uncompressed_len , compressed : PayloadU24 :: from (Payload :: Borrowed (self . compressed . as_ref ())) , } } }
};
}
