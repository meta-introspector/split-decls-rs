// Generated macro for Version (enum)
macro_rules! Depcrate_quicVersion {
() => {
// Module: crate::quic
// Provides: {"Version"}
// Dependencies: {}
# [doc = " QUIC protocol version"] # [doc = ""] # [doc = " Governs version-specific behavior in the TLS layer"] # [non_exhaustive] # [derive (Clone , Copy , Debug , Default)] pub enum Version { # [doc = " First stable RFC"] # [default] V1 , # [doc = " Anti-ossification variant of V1"] V2 , }
};
}
