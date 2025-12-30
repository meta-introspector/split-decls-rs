// Generated macro for Encoding (enum)
macro_rules! Depcrate_msgs_handshakeEncoding {
() => {
// Module: crate::msgs::handshake
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " The method of encoding to use for a handshake message."] # [doc = ""] # [doc = " In some cases a handshake message may be encoded differently depending on the purpose"] # [doc = " the encoded message is being used for."] pub (crate) enum Encoding { # [doc = " Standard RFC 8446 encoding."] Standard , # [doc = " Encoding for ECH confirmation for HRR."] EchConfirmation , # [doc = " Encoding for ECH inner client hello."] EchInnerHello { to_compress : Vec < ExtensionType > } , }
};
}
