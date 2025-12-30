// Generated macro for Tls12Version (struct)
macro_rules! Depcrate_versionsTls12Version {
() => {
// Module: crate::versions
// Provides: {"Tls12Version"}
// Dependencies: {}
# [doc = " Internal data for handling the TLS1.2 protocol."] # [doc = ""] # [doc = " There is one value of this type.  It is `TLS12_VERSION`."] # [non_exhaustive] # [derive (Debug)] pub struct Tls12Version { pub (crate) client : & 'static dyn crate :: client :: ClientHandler < Tls12CipherSuite > , pub (crate) server : & 'static dyn crate :: server :: ServerHandler < Tls12CipherSuite > , }
};
}
