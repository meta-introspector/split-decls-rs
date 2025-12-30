// Generated macro for SupportedProtocolVersions (struct)
macro_rules! Depcrate_msgs_handshakeSupportedProtocolVersions {
() => {
// Module: crate::msgs::handshake
// Provides: {"SupportedProtocolVersions"}
// Dependencies: {}
# [doc = " The body of the `SupportedVersions` extension when it appears in a"] # [doc = " `ClientHello`"] # [doc = ""] # [doc = " This is documented as a preference-order vector, but we (as a server)"] # [doc = " ignore the preference of the client."] # [doc = ""] # [doc = " RFC8446: `ProtocolVersion versions<2..254>;`"] # [derive (Clone , Copy , Debug , Default)] pub (crate) struct SupportedProtocolVersions { pub (crate) tls13 : bool , pub (crate) tls12 : bool , }
};
}
