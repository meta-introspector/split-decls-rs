// Generated macro for impl_513 (impl)
macro_rules! Depcrate_os_macos_secure_transportimpl_513 {
() => {
// Module: crate::os::macos::secure_transport
// Provides: {"impl_513"}
// Dependencies: {}
impl < S > MidHandshakeSslStreamExt for MidHandshakeSslStream < S > { fn client_hello_received (& self) -> bool { self . error () . code () == errSSLClientHelloReceived } }
};
}
