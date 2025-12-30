// Generated macro for IoSession (trait)
macro_rules! Depcrate_common_handshakeIoSession {
() => {
// Module: crate::common::handshake
// Provides: {"IoSession"}
// Dependencies: {}
pub (crate) trait IoSession { type Io ; type Session ; fn skip_handshake (& self) -> bool ; fn get_mut (& mut self) -> (& mut TlsState , & mut Self :: Io , & mut Self :: Session , & mut bool) ; fn into_io (self) -> Self :: Io ; }
};
}
