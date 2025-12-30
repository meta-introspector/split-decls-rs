// Generated macro for macro_299 (macro)
macro_rules! Depcrate_msgs_handshakemacro_299 {
() => {
// Module: crate::msgs::handshake
// Provides: {"macro_299"}
// Dependencies: {}
extension_struct ! { # [doc = " A representation of extensions present in a `HelloRetryRequest` message"] pub (crate) struct HelloRetryRequestExtensions <'a > { ExtensionType :: KeyShare => pub (crate) key_share : Option < NamedGroup >, ExtensionType :: Cookie => pub (crate) cookie : Option < PayloadU16 < NonEmpty >>, ExtensionType :: SupportedVersions => pub (crate) supported_versions : Option < ProtocolVersion >, ExtensionType :: EncryptedClientHello => pub (crate) encrypted_client_hello : Option < Payload <'a >>, } + { # [doc = " Records decoding order of records, and controls encoding order."] pub (crate) order : Option < Vec < ExtensionType >>, } }
};
}
