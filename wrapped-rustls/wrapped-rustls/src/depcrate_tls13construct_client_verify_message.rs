// Generated macro for construct_client_verify_message (function)
macro_rules! Depcrate_tls13construct_client_verify_message {
() => {
// Module: crate::tls13
// Provides: {"construct_client_verify_message"}
// Dependencies: {}
# [doc = " Constructs the signature message specified in section 4.4.3 of RFC8446."] pub (crate) fn construct_client_verify_message (handshake_hash : & hash :: Output) -> VerifyMessage { VerifyMessage :: new (handshake_hash , CLIENT_CONSTANT) }
};
}
