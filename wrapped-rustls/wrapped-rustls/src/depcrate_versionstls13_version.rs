// Generated macro for TLS13_VERSION (static)
macro_rules! Depcrate_versionsTLS13_VERSION {
() => {
// Module: crate::versions
// Provides: {"TLS13_VERSION"}
// Dependencies: {}
# [doc = " Internal data for handling the TLS1.3 protocol."] # [doc = ""] # [doc = " This value refers to TLS1.3 protocol handling code.  This means"] # [doc = " that if your program does not refer to this value, all that code"] # [doc = " can be removed by the linker."] pub static TLS13_VERSION : & Tls13Version = & Tls13Version { client : crate :: client :: TLS13_HANDLER , server : crate :: server :: TLS13_HANDLER , } ;
};
}
