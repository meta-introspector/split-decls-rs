// Generated macro for DisconnectCode (enum)
macro_rules! DepcrateDisconnectCode {
() => {
// Module: crate
// Provides: {"DisconnectCode"}
// Dependencies: {}
# [allow (missing_docs)] # [derive (Copy , Clone)] pub enum DisconnectCode { HostNotAllowedToConnect = raw :: SSH_DISCONNECT_HOST_NOT_ALLOWED_TO_CONNECT as isize , ProtocolError = raw :: SSH_DISCONNECT_PROTOCOL_ERROR as isize , KeyExchangeFailed = raw :: SSH_DISCONNECT_KEY_EXCHANGE_FAILED as isize , Reserved = raw :: SSH_DISCONNECT_RESERVED as isize , MacError = raw :: SSH_DISCONNECT_MAC_ERROR as isize , CompressionError = raw :: SSH_DISCONNECT_COMPRESSION_ERROR as isize , ServiceNotAvailable = raw :: SSH_DISCONNECT_SERVICE_NOT_AVAILABLE as isize , ProtocolVersionNotSupported = raw :: SSH_DISCONNECT_PROTOCOL_VERSION_NOT_SUPPORTED as isize , HostKeyNotVerifiable = raw :: SSH_DISCONNECT_HOST_KEY_NOT_VERIFIABLE as isize , ConnectionLost = raw :: SSH_DISCONNECT_CONNECTION_LOST as isize , ByApplication = raw :: SSH_DISCONNECT_BY_APPLICATION as isize , TooManyConnections = raw :: SSH_DISCONNECT_TOO_MANY_CONNECTIONS as isize , AuthCancelledByUser = raw :: SSH_DISCONNECT_AUTH_CANCELLED_BY_USER as isize , NoMoreAuthMethodsAvailable = raw :: SSH_DISCONNECT_NO_MORE_AUTH_METHODS_AVAILABLE as isize , IllegalUserName = raw :: SSH_DISCONNECT_ILLEGAL_USER_NAME as isize , }
};
}
