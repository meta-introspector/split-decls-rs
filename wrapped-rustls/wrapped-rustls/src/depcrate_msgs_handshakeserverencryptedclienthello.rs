// Generated macro for ServerEncryptedClientHello (struct)
macro_rules! Depcrate_msgs_handshakeServerEncryptedClientHello {
() => {
// Module: crate::msgs::handshake
// Provides: {"ServerEncryptedClientHello"}
// Dependencies: {}
# [doc = " Representation of the ECHEncryptedExtensions extension specified in"] # [doc = " [draft-ietf-tls-esni Section 5]."] # [doc = ""] # [doc = " [draft-ietf-tls-esni Section 5]: <https://www.ietf.org/archive/id/draft-ietf-tls-esni-18.html#section-5>"] # [derive (Clone , Debug)] pub (crate) struct ServerEncryptedClientHello { pub (crate) retry_configs : Vec < EchConfigPayload > , }
};
}
