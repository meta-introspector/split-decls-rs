// Generated macro for InboundPlainMessage (struct)
macro_rules! Depcrate_crypto_cipher_inboundInboundPlainMessage {
() => {
// Module: crate::crypto::cipher::inbound
// Provides: {"InboundPlainMessage"}
// Dependencies: {}
# [doc = " A TLS frame, named `TLSPlaintext` in the standard."] # [doc = ""] # [doc = " This inbound type borrows its decrypted payload from the original buffer."] # [doc = " It results from decryption."] # [expect (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct InboundPlainMessage < 'a > { # [doc = " The content type of the message."] pub typ : ContentType , # [doc = " The protocol version of the message."] pub version : ProtocolVersion , # [doc = " The decrypted payload of the message."] pub payload : & 'a [u8] , }
};
}
