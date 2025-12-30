// Generated macro for InboundOpaqueMessage (struct)
macro_rules! Depcrate_crypto_cipher_inboundInboundOpaqueMessage {
() => {
// Module: crate::crypto::cipher::inbound
// Provides: {"InboundOpaqueMessage"}
// Dependencies: {}
# [doc = " A TLS frame, named TLSPlaintext in the standard."] # [doc = ""] # [doc = " This inbound type borrows its encrypted payload from a buffer elsewhere."] # [doc = " It is used for joining and is consumed by decryption."] # [expect (clippy :: exhaustive_structs)] pub struct InboundOpaqueMessage < 'a > { # [doc = " The content type of the message."] pub typ : ContentType , # [doc = " The protocol version of the message."] pub version : ProtocolVersion , # [doc = " The encrypted payload of the message."] pub payload : BorrowedPayload < 'a > , }
};
}
