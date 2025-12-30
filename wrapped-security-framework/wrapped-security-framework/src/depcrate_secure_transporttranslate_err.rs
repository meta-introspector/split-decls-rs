// Generated macro for translate_err (function)
macro_rules! Depcrate_secure_transporttranslate_err {
() => {
// Module: crate::secure_transport
// Provides: {"translate_err"}
// Dependencies: {}
# [cold] fn translate_err (e : & io :: Error) -> OSStatus { match e . kind () { io :: ErrorKind :: NotFound => errSSLClosedGraceful , io :: ErrorKind :: ConnectionReset => errSSLClosedAbort , io :: ErrorKind :: WouldBlock | io :: ErrorKind :: NotConnected => errSSLWouldBlock , _ => errSecIO , } }
};
}
