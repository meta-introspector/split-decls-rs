// Generated macro for OutboundOpaqueMessage (struct)
macro_rules! Depcrate_crypto_cipher_outboundOutboundOpaqueMessage {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"OutboundOpaqueMessage"}
// Dependencies: {}
# [doc = " A TLS frame, named `TLSPlaintext` in the standard."] # [doc = ""] # [doc = " This outbound type owns all memory for its interior parts."] # [doc = " It results from encryption and is used for io write."] # [expect (clippy :: exhaustive_structs)] # [derive (Clone , Debug)] pub struct OutboundOpaqueMessage { # [doc = " The content type of this message."] pub typ : ContentType , # [doc = " The protocol version of this message."] pub version : ProtocolVersion , # [doc = " The payload of this message."] pub payload : PrefixedPayload , }
};
}
