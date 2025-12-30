// Generated macro for EncryptedClientHello (enum)
macro_rules! Depcrate_msgs_handshakeEncryptedClientHello {
() => {
// Module: crate::msgs::handshake
// Provides: {"EncryptedClientHello"}
// Dependencies: {}
# [doc = " Representation of the `ECHClientHello` client extension specified in"] # [doc = " [draft-ietf-tls-esni Section 5]."] # [doc = ""] # [doc = " [draft-ietf-tls-esni Section 5]: <https://www.ietf.org/archive/id/draft-ietf-tls-esni-18.html#section-5>"] # [derive (Clone , Debug)] pub (crate) enum EncryptedClientHello { # [doc = " A `ECHClientHello` with type [EchClientHelloType::ClientHelloOuter]."] Outer (EncryptedClientHelloOuter) , # [doc = " An empty `ECHClientHello` with type [EchClientHelloType::ClientHelloInner]."] # [doc = ""] # [doc = " This variant has no payload."] Inner , }
};
}
