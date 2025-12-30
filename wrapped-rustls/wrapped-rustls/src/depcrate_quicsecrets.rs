// Generated macro for Secrets (struct)
macro_rules! Depcrate_quicSecrets {
() => {
// Module: crate::quic
// Provides: {"Secrets"}
// Dependencies: {}
# [doc = " Secrets used to encrypt/decrypt traffic"] # [derive (Clone)] pub struct Secrets { # [doc = " Secret used to encrypt packets transmitted by the client"] pub (crate) client : OkmBlock , # [doc = " Secret used to encrypt packets transmitted by the server"] pub (crate) server : OkmBlock , # [doc = " Cipher suite used with these secrets"] suite : & 'static Tls13CipherSuite , quic : & 'static dyn Algorithm , side : Side , version : Version , }
};
}
