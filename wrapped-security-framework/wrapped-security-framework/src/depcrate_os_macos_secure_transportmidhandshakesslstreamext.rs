// Generated macro for MidHandshakeSslStreamExt (trait)
macro_rules! Depcrate_os_macos_secure_transportMidHandshakeSslStreamExt {
() => {
// Module: crate::os::macos::secure_transport
// Provides: {"MidHandshakeSslStreamExt"}
// Dependencies: {}
# [doc = " An extension trait adding OSX specific functionality to the"] # [doc = " `MidHandshakeSslStream` type."] pub trait MidHandshakeSslStreamExt { # [doc = " Returns `true` iff `break_on_client_hello` was set and the handshake"] # [doc = " has progressed to that point."] fn client_hello_received (& self) -> bool ; }
};
}
