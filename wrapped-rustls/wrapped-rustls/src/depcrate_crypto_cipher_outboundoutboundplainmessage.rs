// Generated macro for OutboundPlainMessage (struct)
macro_rules! Depcrate_crypto_cipher_outboundOutboundPlainMessage {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"OutboundPlainMessage"}
// Dependencies: {}
# [doc = " A TLS frame, named `TLSPlaintext` in the standard."] # [doc = ""] # [doc = " This outbound type borrows its \"to be encrypted\" payload from the \"user\"."] # [doc = " It is used for fragmenting and is consumed by encryption."] # [expect (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct OutboundPlainMessage < 'a > { # [doc = " The content type of this message."] pub typ : ContentType , # [doc = " The protocol version of this message."] pub version : ProtocolVersion , # [doc = " The payload of this message."] pub payload : OutboundChunks < 'a > , }
};
}
