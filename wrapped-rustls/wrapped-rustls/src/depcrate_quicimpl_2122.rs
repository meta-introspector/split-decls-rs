// Generated macro for impl_2122 (impl)
macro_rules! Depcrate_quicimpl_2122 {
() => {
// Module: crate::quic
// Provides: {"impl_2122"}
// Dependencies: {}
impl DirectionalKeys { pub (crate) fn new (suite : & 'static Tls13CipherSuite , quic : & 'static dyn Algorithm , secret : & OkmBlock , version : Version ,) -> Self { let builder = KeyBuilder :: new (secret , version , quic , suite . hkdf_provider) ; Self { header : builder . header_protection_key () , packet : builder . packet_key () , } } }
};
}
