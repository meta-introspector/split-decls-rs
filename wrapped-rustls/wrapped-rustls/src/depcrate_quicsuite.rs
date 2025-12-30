// Generated macro for Suite (struct)
macro_rules! Depcrate_quicSuite {
() => {
// Module: crate::quic
// Provides: {"Suite"}
// Dependencies: {}
# [doc = " Produces QUIC initial keys from a TLS 1.3 ciphersuite and a QUIC key generation algorithm."] # [non_exhaustive] # [derive (Clone , Copy)] pub struct Suite { # [doc = " The TLS 1.3 ciphersuite used to derive keys."] pub suite : & 'static Tls13CipherSuite , # [doc = " The QUIC key generation algorithm used to derive keys."] pub quic : & 'static dyn Algorithm , }
};
}
