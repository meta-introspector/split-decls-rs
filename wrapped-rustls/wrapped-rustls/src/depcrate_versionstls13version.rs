// Generated macro for Tls13Version (struct)
macro_rules! Depcrate_versionsTls13Version {
() => {
// Module: crate::versions
// Provides: {"Tls13Version"}
// Dependencies: {}
# [doc = " Internal data for handling the TLS1.3 protocol."] # [doc = ""] # [doc = " There is one value of this type.  It is `TLS13_VERSION`."] # [non_exhaustive] # [derive (Debug)] pub struct Tls13Version { pub (crate) client : & 'static dyn crate :: client :: ClientHandler < Tls13CipherSuite > , pub (crate) server : & 'static dyn crate :: server :: ServerHandler < Tls13CipherSuite > , }
};
}
