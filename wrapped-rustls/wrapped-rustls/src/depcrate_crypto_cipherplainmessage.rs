// Generated macro for PlainMessage (struct)
macro_rules! Depcrate_crypto_cipherPlainMessage {
() => {
// Module: crate::crypto::cipher
// Provides: {"PlainMessage"}
// Dependencies: {}
# [doc = " A decrypted TLS frame"] # [doc = ""] # [doc = " This type owns all memory for its interior parts. It can be decrypted from an OpaqueMessage"] # [doc = " or encrypted into an OpaqueMessage, and it is also used for joining and fragmenting."] # [expect (clippy :: exhaustive_structs)] # [derive (Clone , Debug)] pub struct PlainMessage { # [doc = " The content type of this message."] pub typ : ContentType , # [doc = " The protocol version of this message."] pub version : ProtocolVersion , # [doc = " The payload of this message."] pub payload : Payload < 'static > , }
};
}
