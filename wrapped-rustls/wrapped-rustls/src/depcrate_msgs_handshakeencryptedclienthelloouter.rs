// Generated macro for EncryptedClientHelloOuter (struct)
macro_rules! Depcrate_msgs_handshakeEncryptedClientHelloOuter {
() => {
// Module: crate::msgs::handshake
// Provides: {"EncryptedClientHelloOuter"}
// Dependencies: {}
# [doc = " Representation of the ECHClientHello extension with type outer specified in"] # [doc = " [draft-ietf-tls-esni Section 5]."] # [doc = ""] # [doc = " [draft-ietf-tls-esni Section 5]: <https://www.ietf.org/archive/id/draft-ietf-tls-esni-18.html#section-5>"] # [derive (Clone , Debug)] pub (crate) struct EncryptedClientHelloOuter { # [doc = " The cipher suite used to encrypt ClientHelloInner. Must match a value from"] # [doc = " ECHConfigContents.cipher_suites list."] pub cipher_suite : HpkeSymmetricCipherSuite , # [doc = " The ECHConfigContents.key_config.config_id for the chosen ECHConfig."] pub config_id : u8 , # [doc = " The HPKE encapsulated key, used by servers to decrypt the corresponding payload field."] # [doc = " This field is empty in a ClientHelloOuter sent in response to a HelloRetryRequest."] pub enc : PayloadU16 , # [doc = " The serialized and encrypted ClientHelloInner structure, encrypted using HPKE."] pub payload : PayloadU16 < NonEmpty > , }
};
}
