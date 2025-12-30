// Generated macro for TLS12_VERSION (static)
macro_rules! Depcrate_versionsTLS12_VERSION {
() => {
// Module: crate::versions
// Provides: {"TLS12_VERSION"}
// Dependencies: {}
# [doc = " Internal data for handling the TLS1.2 protocol."] # [doc = ""] # [doc = " This value refers to TLS1.2 protocol handling code.  This means"] # [doc = " that if your program does not refer to this value, all that code"] # [doc = " can be removed by the linker."] pub static TLS12_VERSION : & Tls12Version = & Tls12Version { client : crate :: client :: TLS12_HANDLER , server : crate :: server :: TLS12_HANDLER , } ;
};
}
